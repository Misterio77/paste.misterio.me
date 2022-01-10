{ lib, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "paste-misterio-.me";
  version = "0.1.2";

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
    description = "Pasting service";
    homepage = "https://github.com/Misterio77/paste.misterio.me";
    license = licenses.unlicense;
    platforms = platforms.all;
  };
}
