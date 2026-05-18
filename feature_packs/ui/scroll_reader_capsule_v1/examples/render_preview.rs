use scroll_reader_capsule_v1::{CapsuleMode, sample_capsule};

fn main() {
    let mut capsule = sample_capsule();
    capsule.step_by(3);
    println!("{}", capsule.render_svg(CapsuleMode::Large));
}
