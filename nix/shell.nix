{ pkgs, ... }:
pkgs.mkShell {
  inputsFrom = [
    (pkgs.callPackage ./server.nix { })
    (pkgs.callPackage ./cli.nix { })
  ];
  buildInputs = with pkgs; [
    # Rust tooling
    rustc
    rust-analyzer
    rustfmt
    clippy
    cargo-msrv
    # Postgres tooling
    postgresql
    pgformatter
    sqls
    # Sass tooling
    nodePackages.sass
  ];
}
