use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use feature_core::{FeatureLabError, FeatureLabResult, FeatureManifest, load_feature_manifest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct RegisteredFeature {
    pub manifest: FeatureManifest,
    pub package_name: String,
    pub feature_dir: PathBuf,
    pub feature_toml_path: PathBuf,
    pub cargo_toml_path: PathBuf,
    pub readme_path: PathBuf,
    pub fixtures_dir: PathBuf,
    pub tests_dir: PathBuf,
}

impl RegisteredFeature {
    pub fn category(&self) -> String {
        self.manifest
            .id
            .split('.')
            .next()
            .unwrap_or("uncategorized")
            .to_string()
    }

    pub fn readme_text(&self) -> FeatureLabResult<String> {
        fs::read_to_string(&self.readme_path)
            .map_err(|error| FeatureLabError::io(Some(self.readme_path.clone()), error))
    }

    pub fn first_fixture_preview(&self) -> FeatureLabResult<Option<String>> {
        if !self.fixtures_dir.exists() {
            return Ok(None);
        }
        let mut fixture_files = fs::read_dir(&self.fixtures_dir)
            .map_err(|error| FeatureLabError::io(Some(self.fixtures_dir.clone()), error))?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path())
            .collect::<Vec<_>>();
        fixture_files.sort();
        match fixture_files.first() {
            Some(path) => fs::read_to_string(path)
                .map(Some)
                .map_err(|error| FeatureLabError::io(Some(path.to_path_buf()), error)),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeatureRegistry {
    workspace_root: PathBuf,
    features: Vec<RegisteredFeature>,
}

impl FeatureRegistry {
    pub fn empty(workspace_root: PathBuf) -> Self {
        Self {
            workspace_root,
            features: Vec::new(),
        }
    }

    pub fn discover() -> FeatureLabResult<Self> {
        Self::load_from_root(default_workspace_root())
    }

    pub fn load_from_root(workspace_root: impl Into<PathBuf>) -> FeatureLabResult<Self> {
        let workspace_root = workspace_root.into();
        let feature_root = workspace_root.join("feature_packs");
        let mut manifests = Vec::new();
        collect_feature_tomls(&feature_root, &mut manifests)?;
        let mut features = manifests
            .into_iter()
            .map(|manifest_path| build_registered_feature(&manifest_path))
            .collect::<FeatureLabResult<Vec<_>>>()?;
        features.sort_by(|left, right| left.manifest.id.cmp(&right.manifest.id));
        Ok(Self {
            workspace_root,
            features,
        })
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn features(&self) -> &[RegisteredFeature] {
        &self.features
    }

    pub fn get(&self, feature_id: &str) -> Option<&RegisteredFeature> {
        self.features
            .iter()
            .find(|feature| feature.manifest.id == feature_id)
    }

    pub fn search(&self, query: &str) -> Vec<&RegisteredFeature> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return self.features.iter().collect();
        }
        self.features
            .iter()
            .filter(|feature| {
                let haystack = format!(
                    "{} {} {} {}",
                    feature.manifest.id,
                    feature.manifest.name,
                    feature.manifest.summary,
                    feature.manifest.tags.join(" ")
                )
                .to_lowercase();
                haystack.contains(&query)
            })
            .collect()
    }
}

pub fn default_workspace_root() -> PathBuf {
    if let Ok(value) = env::var("FEATURE_LAB_ROOT") {
        return PathBuf::from(value);
    }
    if let Ok(current_dir) = env::current_dir() {
        if let Some(root) = find_workspace_root_from(&current_dir) {
            return root;
        }
    }
    find_workspace_root_from(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("workspace root should exist")
}

fn find_workspace_root_from(start: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .find(|candidate| {
            candidate.join("Cargo.toml").is_file() && candidate.join("feature_packs").is_dir()
        })
        .map(Path::to_path_buf)
}

fn collect_feature_tomls(root: &Path, manifests: &mut Vec<PathBuf>) -> FeatureLabResult<()> {
    if !root.exists() {
        return Ok(());
    }
    for entry in
        fs::read_dir(root).map_err(|error| FeatureLabError::io(Some(root.to_path_buf()), error))?
    {
        let entry = entry.map_err(FeatureLabError::from)?;
        let path = entry.path();
        if path.is_dir() {
            collect_feature_tomls(&path, manifests)?;
        } else if path.file_name().is_some_and(|name| name == "feature.toml") {
            manifests.push(path);
        }
    }
    Ok(())
}

fn build_registered_feature(feature_toml_path: &Path) -> FeatureLabResult<RegisteredFeature> {
    let manifest = load_feature_manifest(feature_toml_path)?;
    let feature_dir = feature_toml_path
        .parent()
        .ok_or_else(|| {
            FeatureLabError::Validation("feature.toml must have a parent directory".into())
        })?
        .to_path_buf();
    let cargo_toml_path = feature_dir.join("Cargo.toml");
    let readme_path = feature_dir.join("README.md");
    let fixtures_dir = feature_dir.join("fixtures");
    let tests_dir = feature_dir.join("tests");
    let package_name = cargo_package_name(&cargo_toml_path)?;
    Ok(RegisteredFeature {
        manifest,
        package_name,
        feature_dir,
        feature_toml_path: feature_toml_path.to_path_buf(),
        cargo_toml_path,
        readme_path,
        fixtures_dir,
        tests_dir,
    })
}

fn cargo_package_name(path: &Path) -> FeatureLabResult<String> {
    let raw = fs::read_to_string(path)
        .map_err(|error| FeatureLabError::io(Some(path.to_path_buf()), error))?;
    let manifest: CargoManifest = toml::from_str(&raw)
        .map_err(|error| FeatureLabError::toml(Some(path.to_path_buf()), error))?;
    Ok(manifest.package.name)
}

#[derive(Debug, Deserialize)]
struct CargoManifest {
    package: CargoPackage,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
}

#[cfg(test)]
mod tests {
    use super::{FeatureRegistry, default_workspace_root, find_workspace_root_from};

    #[test]
    fn discovers_right_inspector_feature() {
        let registry = FeatureRegistry::discover().expect("registry should load");
        let feature = registry
            .get("ui.right_inspector")
            .expect("feature should exist");
        assert_eq!(feature.package_name, "right_inspector");
    }

    #[test]
    fn finds_workspace_root_from_nested_path() {
        let root = default_workspace_root();
        let nested = root.join("feature_packs/ui/right_inspector/src");
        assert_eq!(find_workspace_root_from(&nested), Some(root));
    }
}
