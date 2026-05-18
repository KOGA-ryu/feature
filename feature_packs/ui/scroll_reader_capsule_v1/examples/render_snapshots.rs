use std::{env, fs, path::PathBuf};

use scroll_reader_capsule_v1::{
    CapsuleMode, CapsulePalette, ScrollReaderCapsule, render_plan_svg, sample_capsule,
};

fn main() {
    let output_dir = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("fixtures/render_snapshots"));
    fs::create_dir_all(&output_dir).expect("snapshot output directory should be creatable");

    let snapshots = [
        ("large_mid.svg", large_mid()),
        ("compact_mid.svg", compact_mid()),
        ("large_start.svg", large_start()),
        ("large_end.svg", large_end()),
        ("compact_empty.svg", compact_empty()),
        ("large_long_focus_word.svg", large_long_focus_word()),
        ("large_transition_50.svg", large_transition_50()),
    ];

    for (file_name, svg) in snapshots {
        fs::write(output_dir.join(file_name), format!("{svg}\n"))
            .expect("snapshot fixture should be writable");
    }
}

fn large_mid() -> String {
    let mut capsule = sample_capsule();
    capsule.step_by(3);
    capsule.render_svg(CapsuleMode::Large)
}

fn compact_mid() -> String {
    let mut capsule = sample_capsule();
    capsule.step_by(3);
    capsule.render_svg(CapsuleMode::Compact)
}

fn large_start() -> String {
    sample_capsule().render_svg(CapsuleMode::Large)
}

fn large_end() -> String {
    let mut capsule = sample_capsule();
    capsule.step_by(999);
    capsule.render_svg(CapsuleMode::Large)
}

fn compact_empty() -> String {
    ScrollReaderCapsule::from_selected_text(" \n\t ").render_svg(CapsuleMode::Compact)
}

fn large_long_focus_word() -> String {
    let mut capsule = ScrollReaderCapsule::from_selected_text(
        "before context pneumonoultramicroscopicsilicovolcanoconiosis after context remains readable",
    );
    capsule.step_by(2);
    capsule.render_svg(CapsuleMode::Large)
}

fn large_transition_50() -> String {
    let capsule = sample_capsule();
    render_plan_svg(
        &capsule.render_transition_plan(CapsuleMode::Large, 3, 4, 500),
        &CapsulePalette::default(),
    )
}
