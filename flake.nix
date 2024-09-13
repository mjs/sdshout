{
  description = "sdshout";

  inputs = {
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs {inherit system;};
        toolchain = pkgs.rustPlatform;
      in rec
      {
        # Executed by `nix build`
        packages.default = toolchain.buildRustPackage {
          pname = "sdshout";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          buildInputs = with pkgs; [pkg-config dbus];
          nativeBuildInputs = with pkgs; [pkg-config dbus];
        };

        # Executed by `nix run`
        apps.default = utils.lib.mkApp {drv = packages.default;};

        # Used by `nix develop`
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            (with toolchain; [
              cargo
              rustc
              rustLibSrc
            ])
            rust-analyzer
            clippy
            rustfmt
            cargo-watch
            pkg-config
            dbus
          ];

          # Specify the rust-src path (many editors rely on this)
          RUST_SRC_PATH = "${toolchain.rustLibSrc}";
        };
      }
    );
}
