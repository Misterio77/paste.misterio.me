{ pkgs, ... }: let
  server = pkgs.callPackage ./server { };
  cli = pkgs.callPackage ./cli { };
in
pkgs.mkShell {
  inherit (server) PICO_PATH TURBO_PATH SYNTAXES_PATH;
  inputsFrom = [ server cli ];
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
