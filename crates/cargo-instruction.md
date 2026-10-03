# Crates.io Publishing Instructions

This guide outlines the steps to publish the **ApexCode** workspace crates to
[crates.io](https://crates.io).

## 1. Prerequisites

- [ ] Create an account on [crates.io](https://crates.io).
- [ ] Generate an API token from your account settings.
- [ ] Log in locally:
  ```bash
  cargo login <your-api-token>
  ```

## 2. Publishing Sequence

Because the crates depend on each other, they **must** be published in the following order. Wait a
few seconds between each command to allow the registry to index the new version.

### Phase 1: Core Foundation

```bash
cargo publish -p apexcode-core
```

### Phase 2: Functional Engines

Wait ~10 seconds, then run:

```bash
cargo publish -p apexcode-detector
cargo publish -p apexcode-humanizer
cargo publish -p apexcode-jitter
cargo publish -p apexcode-undercover
```

### Phase 3: User Interface

```bash
cargo publish -p apexcode-tui
```

### Phase 4: Main CLI

```bash
cargo publish -p apexcode
```

## 3. Verification Commands

Before actual publishing, you can run a dry-run to ensure everything is correct:

```bash
# Check the whole workspace
cargo publish -p apexcode-core --dry-run
# ... repeat for other packages if needed
```

## 4. Troubleshooting

- **Dirty Repository**: `cargo publish` will fail if you have uncommitted changes. Commit your work
  first.
- **Documentation**: Ensure `README.md` and `LICENSE` are present in the root (they are
  automatically included in the package).
- **Version Mismatch**: If you update one crate, you must update the version in the root
  `Cargo.toml` and ensure all internal dependencies match.

---
