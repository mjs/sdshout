{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, utils, nixpkgs, fenix, }: utils.lib.eachDefaultSystem (system: let 
    pkgs = nixpkgs.legacyPackages.${system};
    rust = fenix.packages.${system};
    lib = pkgs.lib;
  in {
    devShell = pkgs.mkShell {
      buildInputs = with pkgs; with llvmPackages; [
        rust.stable.toolchain pkg-config dbus
      ];

      RUST_BACKTRACE = 1;
      # RUST_LOG = "info,sqlx::query=warn";
      # RUSTFLAGS = "-C target-cpu=native";
    };
  });
}
