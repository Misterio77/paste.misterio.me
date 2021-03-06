{ lib, rustPlatform }:

let manifest = (lib.importTOML ./Cargo.toml).package;
in rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  postInstall = ''
    mkdir -p $out/etc
    cp -r templates db $out/etc
  '';

  meta = with lib; {
    description = manifest.description;
    homepage = manifest.homepage;
    license = licenses.agpl3Plus;
    platforms = platforms.all;
  };
}
