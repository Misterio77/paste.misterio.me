{ lib, rustPlatform }:

let manifest = (lib.importTOML ../server/Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  src = lib.sourceByRegex ../. [ "Cargo.toml" "Cargo.lock" "cli.*" "server.*" ];

  cargoLock.lockFile = ../Cargo.lock;

  buildAndTestSubdir = "server";

  postInstall = ''
    mkdir -p $out/etc
    cp -r server/templates server/db $out/etc
  '';

  meta = with lib; {
    description = manifest.description;
    homepage = manifest.homepage;
    license = licenses.agpl3Plus;
    platforms = platforms.all;
  };
}
