use feature_core::{FeatureLabError, FeatureLabResult};
use feature_registry::FeatureRegistry;

use crate::command_runner::{CommandOutput, run_command};

pub fn run_feature_tests(
    registry: &FeatureRegistry,
    feature_id: &str,
) -> FeatureLabResult<CommandOutput> {
    let feature = registry
        .get(feature_id)
        .ok_or_else(|| FeatureLabError::NotFound(format!("unknown feature '{feature_id}'")))?;
    run_command(
        "cargo",
        &["test", "-p", feature.package_name.as_str()],
        registry.workspace_root(),
    )
}
