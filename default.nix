{ lib, rustPlatform }:

let manifest = (lib.importTOML ./Cargo.toml).package;
in rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "rocket-0.5.0-rc.1" = "sha256-wmC/nekpOx7Dwy4dRVoEWxrznnlw9r3Nmq8J9X+Kbmo=";
    };
  };

  postInstall = ''
    mkdir -p $out/etc
    cp -r templates db $out/etc
  '';

  meta = with lib; {
    description = manifest.desciption;
    homepage = manifest.homepage;
    license = licenses.agpl3Plus;
    platforms = platforms.all;
  };
}
