{ lib, rustPlatform, nodePackages }:

rustPlatform.buildRustPackage rec {
  pname = "auth-demo";
  version = "0.1.0";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "rocket-0.5.0-rc.1" = "sha256-wmC/nekpOx7Dwy4dRVoEWxrznnlw9r3Nmq8J9X+Kbmo=";
    };
  };

  postInstall = ''
    mkdir -p $out/etc/assets
    cp assets/style.css $out/etc/assets
    cp -r templates db $out/etc
  '';

  meta = with lib; {
    description = "Auth MVC example";
    homepage = "https://github.com/Misterio77/auth-demo";
    license = licenses.unlicense;
    platforms = platforms.all;
  };
}