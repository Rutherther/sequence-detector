{ lib
, stdenv
, clangStdenv
, rustPlatform
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
rustPlatform.buildRustPackage rec {
  src = ./.;
  cargoHash = "sha256-mXkNN5FJphPW/5cn3Fv/Nsfjkc3tssgcZZmHWCfEeYE=";

  name = "${cargoToml.package.name}";
  version = "${cargoToml.package.version}";

  nativeBuildInputs = [
    rustfmt
    pkg-config
    cargo
    rustc
  ];

  checkInputs = [ cargo rustc ];
  doCheck = true;

  meta = {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = [ lib.licenses.mit ];
    maintainers = [];
  };
}
