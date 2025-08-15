## sway-focus-flash

Small Sway utility that briefly animates the opacity of the newly focused window so your eye can find it faster.

It subscribes to Sway IPC events and, on focus changes, runs a short, configurable fade on the focused container.

### Features

- Brief opacity "flash" when focus changes
- Customizable easing, frame time, and step count
- Lightweight

## Install

- Download the latest prebuilt binary from the project’s Releases page and put it on your PATH (e.g. `~/.local/bin`).
- Or build from source:

```bash
# System dependencies: Rust (stable), cargo
cargo build --release
# Then copy/install
install -Dm755 target/release/sway-focus-flash ~/.local/bin/sway-focus-flash
```

If you use Nix with devenv/direnv, you can enter a dev shell first, then build:

```bash
devenv shell  
cargo build --release
```

### Install and run with Nix (flakes)

- Run directly (no install):

```bash
# Latest from GitHub
nix run github:perstarkse/sway-focus-flash

# Optionally pin a ref
nix run github:perstarkse/sway-focus-flash?ref=main
```

- Use as a flake input in your Nix(OS)/Home Manager config:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    sway-focus-flash.url = "github:perstarkse/sway-focus-flash";
  };

  # ... expose inputs.sway-focus-flash to your Home Manager module scope ...
}
```

## Usage

Run the binary; it will stay in the background listening to IPC events:

```bash
sway-focus-flash
```

### Autostart (recommended)

- Plain Sway config:

```conf
# ~/.config/sway/config
exec_always --no-startup-id sway-focus-flash
```

- Home Manager (flake) inside your Sway module:

```nix
# Minimal example inside your wayland.windowManager.sway.config
{
  wayland.windowManager.sway = {
    enable = true;
    config.startup = [
      {
        command = "${inputs.sway-focus-flash.packages.${pkgs.system}.sway-focus-flash}/bin/sway-focus-flash";
        always = true;
      }
    ];
  };
}
```

### Configuration (CLI flags)

All settings are provided as command-line flags. Defaults are shown in brackets.

```text
--start-opacity <FLOAT>   Starting opacity 0.0–1.0 [0.8]
--end-opacity   <FLOAT>   Ending opacity   0.0–1.0 [1.0]
--steps         <INT>     Number of frames per animation [30]
--frame-time    <MS>      Duration of each frame in milliseconds [20]
--ease          <EASING>  Easing function [ease-in-out-quint]
                          Values: linear | ease-in-cubic | ease-out-cubic |
                                  ease-in-out-cubic | ease-out-quint | ease-in-out-quint
```

Examples:

```bash
# Slower, smoother flash
sway-focus-flash --steps 48 --frame-time 24 --ease ease-in-out-cubic

# More subtle start
sway-focus-flash --start-opacity 0.9
```

## How it works

- Subscribes to Sway `window` and `workspace` events
- On focus change, animates the focused container’s opacity from `start-opacity` to `end-opacity`
- On workspace change, resets opacities and triggers a flash for the currently focused container

This relies on Sway’s `opacity` command for containers. To verify your setup supports it, try:

```bash
swaymsg '[con_id=__focused__]' opacity 0.9
```

If that command works and returns success, `sway-focus-flash` will work in your session.

## Sway tips

- If you’re starting the utility with `exec_always`, ensure you don’t start multiple copies elsewhere (bars, scripts, etc.).
- If another script or style automation is also changing `opacity`, they may conflict. Prefer a single source of truth.

## Troubleshooting

- Nothing happens at all: check that `swaymsg '[con_id=__focused__]' opacity 0.9` works, and that the binary is running (`pgrep -fa sway-focus-flash`).
- Waybar or other tools also manipulate window visuals: disable overlapping features to avoid flicker.

## Updating

- Download the latest release and replace your local binary, or rebuild with cargo.

## Contributing

- Issues and PRs are welcome. Please keep the utility small and focused on the “find the focused window” use case.

## Acknowledgements

Inspired by small Sway/i3 helper tools that improve focus visibility by briefly changing window state or appearance.

---

Repository: [perstarkse/sway-focus-flash](https://github.com/perstarkse/sway-focus-flash)
