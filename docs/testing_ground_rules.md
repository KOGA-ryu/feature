# Testing Ground Rules

1. Every feature must be testable in isolation.
2. `cargo test -p <feature-crate>` should prove the feature contract before any UI polish.
3. Fixtures belong with the feature crate that uses them.
4. Shared crates may be changed only when the feature cannot stay isolated otherwise.
5. The workspace should stay runnable without external services.
