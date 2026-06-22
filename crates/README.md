# Crates

## [{{project-name}}](./{{project-name}})

Public library and optional executable entrypoint.

## [xtask](./xtask)

Development automation. Not published.

| Command                           | What it does                                      |
| --------------------------------- | ------------------------------------------------- |
| `cargo xtask check`               | Format, compile, and lint                          |
| `cargo xtask test [filter]`       | Check and run tests                                |
| `cargo xtask build`               | Test and build release artifacts                   |
| `cargo xtask add <name>`          | Add a library; reserved `cli` adds the project CLI |
| `cargo xtask coverage`            | Generate HTML coverage                             |
| `cargo xtask publish [--execute]` | Prepare a lockstep workspace release               |
