## Build Workflow

- Source `build/env.sh` before using project tooling so `bear` is on `PATH`.
- Prefer `bear fmt` for formatting.
- Prefer `bear build <target>` for firmware builds and validation instead of invoking Cargo directly.
- Use `bear target list` to discover supported targets.
