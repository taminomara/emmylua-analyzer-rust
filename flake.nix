{
  description = "EmmyLua Language Server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        cargoToml =
          builtins.fromTOML (builtins.readFile ./crates/emmylua_ls/Cargo.toml);
      in {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;

          src = ./.;
          cargoLock = { lockFile = ./Cargo.lock; };

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ stdenv.cc.cc.lib ];

          buildAndTestSubdir = "crates/emmylua_ls";

          postFixup = ''
            patchelf --set-rpath "${pkgs.stdenv.cc.cc.lib}/lib" $out/bin/emmylua_ls
          '';
        };

        packages.emmylua_ls = self.packages.${system}.default;
      });
}
