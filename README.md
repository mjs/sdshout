# sdshout ("systemd shout")

sdshout is a simple service which generates desktop notifications on Linux
systems when systemd services fail. It's useful for knowing when system
updates, automated backups or other critical jobs aren't working.

sdshout is intended to run a user systemd service. It connects to the system
D-Bus to detect failed services.


## Installation

### Nix

On flake enabled Nix systems, sdshout can be run or installed from
`github:mjs/sdshout`. For example, to just give it a try:

```
nix run github:mjs/sdshout
```

Integration as a NixOS flake input is also possible.

### Cargo

If you have Rust toolchain available, sdshout can be installed with:

```
cargo install sdshout
```

### Service Installation

1. Copy the [sample service file](sdshout.service) to `~/.config/systemd/user`
2. Edit the file and modify the `ExecStart` line to suit
3. `systemctl --user daemon-reload`
4. `systemctl --user enable --now sdshout.service`

## Roadmap

See issues labeled with [Enhancement](https://github.com/mjs/sdshout/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement) on GitHub.
