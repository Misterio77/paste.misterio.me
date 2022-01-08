{
  description = "Auth MVC demo";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    let
      overlay = final: prev: {
        auth-demo = final.callPackage ./default.nix { };
      };
      overlays = [ overlay ];
    in
    {
      inherit overlay overlays;
    } //
    (flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system overlays; };
      in
      rec {
        # nix build
        packages.auth-demo = pkgs.auth-demo;
        defaultPackage = packages.auth-demo;

        # nix run
        apps.auth-demo = flake-utils.lib.mkApp { drv = packages.auth-demo; };
        defaultApp = apps.auth-demo;

        # nix develop
        devShell = pkgs.mkShell {
          inputsFrom = [ defaultPackage ];
          buildInputs = with pkgs; [
            # Rust tooling
            rustc rust-analyzer rustfmt clippy
            # Postgres tooling
            postgresql pgformatter sqls
            # HTML/CSS tooling
            nodePackages.prettier
            sass
          ];
        };
      }));
}

