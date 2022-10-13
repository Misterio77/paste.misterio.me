{ pkgs, ... }:
pkgs.mkShell {
  inputsFrom = [
    (pkgs.callPackage ./server { })
    (pkgs.callPackage ./cli { })
  ];
  buildInputs = with pkgs; [
    # Rust tooling
    rustc
    rust-analyzer
    rustfmt
    clippy
    # Postgres tooling
    postgresql
    pgformatter
    sqls
    # Sass tooling
    nodePackages.sass
  ];
}
