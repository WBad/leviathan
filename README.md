## Dev Environment

With VS Code's remote development extension (ms-vscode-remote.vscode-remote-extensionpack) will allow opening the project's folder in a container, see: `.devcontainer` for environment details

Test: `cargo test`
Build: `cargo build`
Run: `cargo run`

## Release & Run
- `docker build --tag leviathan:latest .`
- `docker run -ti --rm leviathan:latest`
