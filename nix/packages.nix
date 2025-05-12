let
  root = ../.;
  mkPackage =
    x:
    {
      rustPlatform,
      stdenv,
      pkg-config,
    }:
    let
      cargoToml = builtins.fromTOML (builtins.readFile /${root}/crates/${x}/Cargo.toml);
    in
    rustPlatform.buildRustPackage {
      pname = cargoToml.package.name;
      version = cargoToml.package.version;

      src = root;
      cargoLock.lockFile = root + /Cargo.lock;

      nativeBuildInputs = [ pkg-config ];
      buildInputs = [ stdenv.cc.cc.lib ];

      buildAndTestSubdir = "crates/${x}";

      postFixup = ''
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
