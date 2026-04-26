use std::env;
use std::process::ExitCode;

use feature_registry::FeatureRegistry;
use feature_runner::run_feature_tests;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let registry = FeatureRegistry::discover().map_err(|error| error.to_string())?;
    match args.as_slice() {
        [command] if command == "list" => {
            for feature in registry.features() {
                println!(
                    "{}\t{}\t{}\t{}",
                    feature.manifest.id,
                    feature.manifest.name,
                    feature.manifest.status,
                    feature.manifest.summary
                );
            }
            Ok(())
        }
        [command, feature_id] if command == "show" => {
            let feature = registry
                .get(feature_id)
                .ok_or_else(|| format!("unknown feature '{feature_id}'"))?;
            let payload =
                serde_json::to_string_pretty(feature).map_err(|error| error.to_string())?;
            println!("{payload}");
            Ok(())
        }
        [command, feature_id] if command == "test" => {
            let output =
                run_feature_tests(&registry, feature_id).map_err(|error| error.to_string())?;
            println!("command: {}", output.command);
            print!("{}", output.stdout);
            if !output.stderr.trim().is_empty() {
                eprintln!("{}", output.stderr);
            }
            Ok(())
        }
        _ => Err(usage()),
    }
}

fn usage() -> String {
    "usage:\n  cargo run -p feature_cli -- list\n  cargo run -p feature_cli -- show <feature_id>\n  cargo run -p feature_cli -- test <feature_id>".into()
}
