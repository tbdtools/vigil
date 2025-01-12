#!/bin/bash

# Create base directory structure
mkdir -p crates/{vigil-common,vigil-core,vigil-cli,vigil-rules,vigil-events}/src

# Create minimal lib.rs files for each library crate
echo 'pub fn add(left: usize, right: usize) -> usize { left + right }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}' >crates/vigil-common/src/lib.rs

cp crates/vigil-common/src/lib.rs crates/vigil-core/src/lib.rs
cp crates/vigil-common/src/lib.rs crates/vigil-rules/src/lib.rs
cp crates/vigil-common/src/lib.rs crates/vigil-events/src/lib.rs

# Create Cargo.toml files for rules and events crates
cat >crates/vigil-rules/Cargo.toml <<'EOF'
[package]
name = "vigil-rules"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
vigil-common = { path = "../vigil-common" }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }

[dev-dependencies]
test-case = { workspace = true }
EOF

cat >crates/vigil-events/Cargo.toml <<'EOF'
[package]
name = "vigil-events"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
vigil-common = { path = "../vigil-common" }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }

[dev-dependencies]
test-case = { workspace = true }
EOF
