let
  root = ../.;
  mkPackage =
    x:
    {
      lib,
      rustPlatform,
      stdenv,
      pkg-config,
      rust-bin,
    }:
    let
      cargoToml = builtins.fromTOML (builtins.readFile /${root}/crates/${x}/Cargo.toml);
      rustToolchain = rust-bin.stable."1.85.0".default.override {
        extensions = [ "rust-src" "clippy" "rustfmt" ];
      };
      rustPlatformWithToolchain = rustPlatform.override {
        rustc = rustToolchain;
        cargo = rustToolchain;
      };
    in
    rustPlatformWithToolchain.buildRustPackage {
      pname = cargoToml.package.name;
      version = cargoToml.package.version;

      src = root;
      cargoLock.lockFile = root + /Cargo.lock;

      nativeBuildInputs = [ pkg-config ];
      buildInputs = [ stdenv.cc.cc.lib ];

      buildAndTestSubdir = "crates/${x}";

      postFixup = lib.optionalString (!stdenv.hostPlatform.isDarwin) ''
        patchelf --set-rpath "${stdenv.cc.cc.lib}/lib" $out/bin/${x}
      '';
    };
in
(builtins.listToAttrs (
  map
    (
      x:
      let
        name = "emmylua_${x}";
      in
      {
        inherit name;
        value = mkPackage name;
      }
    )
    [
      "ls"
      "doc_cli"
      "check"
    ]
))
