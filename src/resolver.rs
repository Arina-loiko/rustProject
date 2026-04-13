use std::collections::HashMap;

use crate::error::ResolverError;
use crate::graph::DepGraph;
use crate::manifest::Manifest;
use crate::registry::Registry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPackage {
    pub name: String,
    pub version: String,
}

pub fn resolve(manifest: &Manifest, registry: &Registry) -> Result<Vec<ResolvedPackage>, ResolverError> {
    let mut graph = DepGraph::new();
    let mut chosen: HashMap<String, String> = HashMap::new();

    let root_key = format!("{}@{}", manifest.package.name, manifest.package.version);
    let root_id = graph.add_node(&root_key);
    chosen.insert(manifest.package.name.clone(), manifest.package.version.clone());

    let mut stack: Vec<(String, String, usize)> = Vec::new();
    for (dep, ver) in &manifest.dependencies {
        stack.push((dep.clone(), ver.clone(), root_id));
    }

    while let Some((name, version, parent)) = stack.pop() {
        if let Some(existing) = chosen.get(&name) {
            if existing != &version {
                return Err(ResolverError::VersionConflict {
                    name: name.clone(),
                    wanted: version.clone(),
                    existing: existing.clone(),
                });
            }
            let key = format!("{}@{}", name, version);
            let id = graph.add_node(&key);
            graph.add_edge(id, parent);
            continue;
        }
        let info = registry.get(&name, &version).ok_or_else(|| ResolverError::PackageNotFound {
            name: name.clone(),
            version: version.clone(),
        })?;
        let key = format!("{}@{}", name, version);
        let id = graph.add_node(&key);
        graph.add_edge(id, parent);
        chosen.insert(name.clone(), version.clone());
        for (d, v) in &info.dependencies {
            stack.push((d.clone(), v.clone(), id));
        }
    }

    if let Some(cycle) = graph.find_cycle() {
        return Err(ResolverError::Cycle(cycle));
    }

    let order = graph.topo_sort().ok_or_else(|| ResolverError::Cycle(Vec::new()))?;

    let result: Vec<ResolvedPackage> = order
        .into_iter()
        .map(|id| {
            let key = graph.name(id);
            let (n, v) = key.split_once('@').unwrap();
            ResolvedPackage {
                name: n.to_string(),
                version: v.to_string(),
            }
        })
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::PackageInfo;

    fn pkg(deps: &[(&str, &str)]) -> PackageInfo {
        let mut m = HashMap::new();
        for (k, v) in deps {
            m.insert((*k).to_string(), (*v).to_string());
        }
        PackageInfo { dependencies: m }
    }

    fn reg(items: &[(&str, &str, PackageInfo)]) -> Registry {
        let mut r = Registry::default();
        for (name, ver, info) in items {
            let entry = r.entries.entry((*name).to_string()).or_default();
            let cloned = PackageInfo {
                dependencies: info.dependencies.clone(),
            };
            entry.insert((*ver).to_string(), cloned);
        }
        r
    }

    fn manifest(deps: &[(&str, &str)]) -> Manifest {
        let mut m = HashMap::new();
        for (k, v) in deps {
            m.insert((*k).to_string(), (*v).to_string());
        }
        Manifest {
            package: crate::manifest::Package {
                name: "app".to_string(),
                version: "1.0.0".to_string(),
            },
            dependencies: m,
        }
    }

    #[test]
    fn resolves_chain() {
        let r = reg(&[
            ("serde", "1.0.0", pkg(&[("serde_derive", "1.0.0")])),
            ("serde_derive", "1.0.0", pkg(&[])),
        ]);
        let m = manifest(&[("serde", "1.0.0")]);
        let order = resolve(&m, &r).unwrap();
        let names: Vec<&str> = order.iter().map(|p| p.name.as_str()).collect();
        let pos = |n: &str| names.iter().position(|x| *x == n).unwrap();
        assert!(pos("serde_derive") < pos("serde"));
        assert!(pos("serde") < pos("app"));
    }

    #[test]
    fn version_conflict_detected() {
        let r = reg(&[
            ("a", "1.0.0", pkg(&[("c", "1.0.0")])),
            ("b", "1.0.0", pkg(&[("c", "2.0.0")])),
            ("c", "1.0.0", pkg(&[])),
            ("c", "2.0.0", pkg(&[])),
        ]);
        let m = manifest(&[("a", "1.0.0"), ("b", "1.0.0")]);
        let err = resolve(&m, &r).unwrap_err();
        assert!(matches!(err, ResolverError::VersionConflict { .. }));
    }

    #[test]
    fn missing_package_errors() {
        let r = reg(&[]);
        let m = manifest(&[("serde", "1.0.0")]);
        let err = resolve(&m, &r).unwrap_err();
        assert!(matches!(err, ResolverError::PackageNotFound { .. }));
    }
}
