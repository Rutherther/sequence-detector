{ lib
, naersk
, stdenv
, clangStdenv
, hostPlatform
, targetPlatform
, pkg-config
, rustfmt
, cargo
, rustc
}:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in
naersk.lib."${targetPlatform.system}".buildPackage rec {
  src = ./.;
  buildInputs = [
    rustfmt
    pkg-config
    cargo
    rustc
  ];

  checkInputs = [ cargo rustc ];

  doCheck = true;
  copyLibs = true;

  meta = {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = [ lib.licenses.mit ];
    maintainers = [];
  };
}
