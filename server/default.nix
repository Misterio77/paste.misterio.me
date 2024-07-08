{ lib, rustPlatform, mkYarnPackage, fetchFromGitHub, stdenv, fetchYarnDeps }:

let
  manifest = (lib.importTOML ./Cargo.toml).package;
  turbo = mkYarnPackage rec {
    pname = "turbo";
    version = "8.0.4";
    src = fetchFromGitHub {
      owner = "hotwired";
      repo = pname;
      rev = "v${version}";
      hash = "sha256-XxocsGgdGlzs9JOByuM+ujaJTrE/Oycay2dxn+8Qo2I=";
    };
    offlineCache = fetchYarnDeps {
      yarnLock = "${src}/yarn.lock";
      hash = "sha256-BOyKVRKFS+XI0A9Lp9NODxuVFTmmwjWD5Znw8XP8bbs=";
    };
    buildPhase = "yarn --offline build";
    installPhase = "mv ./deps/@hotwired/turbo/dist $out";
    doDist = false;
  };
  picocss = stdenv.mkDerivation rec {
    pname = "pico";
    version = "2.0.0-alpha1";
    src = fetchFromGitHub {
      owner = "picocss";
      repo = pname;
      rev = "v${version}";
      hash = "sha256-GY6B1orGuskpl6U20lU//fZ8axNMgg3XBeHY3RdQfLc=";
    };
    dontBuild = true;
    installPhase = "mkdir -p $out && cp $src/scss $out/pico -r";
  };
  syntaxes = stdenv.mkDerivation {
    pname = "syntaxes";
    version = "unstable-2022-08-23";
    src = fetchFromGitHub {
      owner = "sourcegraph";
      repo = "packages";
      rev = "7481597c6971eec37f0df60a77b2b2a65099fb62";
      hash = "sha256-ZSQpnzBith+tVi0s+MuUNrMtGPLfArXJexcf8Hp+oho=";
    };
    dontBuild = true;
    installPhase = "cp $src $out -r";
  };
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../Cargo.lock
      ../Cargo.toml
      ../cli/src
      ../cli/Cargo.toml
      ./Cargo.toml
      ./build.rs
      ./db
      ./scss
      ./src
      ./templates
    ];
  };
  buildAndTestSubdir = "server";
  cargoLock.lockFile = ../Cargo.lock;

  PICO_PATH = picocss;
  TURBO_PATH = turbo;
  SYNTAXES_PATH = syntaxes;

  postInstall = ''
    mkdir -p $out/etc
    cp -r server/templates server/db $out/etc
  '';

  meta = with lib; {
    mainProgram = "paste-misterio-me";
    description = manifest.description;
    homepage = manifest.homepage;
    license = licenses.agpl3Plus;
    platforms = platforms.all;
  };
}
