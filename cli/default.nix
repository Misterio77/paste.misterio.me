{ lib, rustPlatform, openssl, pkg-config, installShellFiles }:

let manifest = (lib.importTOML ./Cargo.toml).package;
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../Cargo.lock
      ../Cargo.toml
      ../server/src
      ../server/Cargo.toml
      ./Cargo.toml
      ./src
    ];
  };
  buildAndTestSubdir = "cli";
  cargoLock.lockFile = ../Cargo.lock;

  buildInputs = [ openssl ];
  nativeBuildInputs = [ pkg-config installShellFiles ];

  postInstall = ''
    installShellCompletion --cmd pmis \
      --zsh <($out/bin/pmis completions zsh) \
      --fish <($out/bin/pmis completions fish) \
      --bash <($out/bin/pmis completions bash)
  '';

  meta = with lib; {
    description = manifest.description;
    homepage = manifest.homepage;
    license = licenses.gpl3Plus;
    platforms = platforms.all;
  };
}
