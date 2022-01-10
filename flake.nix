{
  description = "Paste service";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      name = "paste-misterio-me";
      overlay = final: prev: {
        ${name} = final.callPackage ./default.nix { };
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
        packages.${name} = pkgs.${name};
        defaultPackage = packages.${name};

        # nix run
        apps.${name} = flake-utils.lib.mkApp { drv = packages.${name}; };
        defaultApp = apps.${name};

        # nix develop
        devShell = pkgs.mkShell {
          inputsFrom = [ defaultPackage ];
          buildInputs = with pkgs; [
            # Rust tooling
            rustc rust-analyzer rustfmt clippy
            # Postgres tooling
            postgresql pgformatter sqls
            # Sass tooling
            nodePackages.sass
            # Httpie for testing
            httpie
          ];
        };
      }));
}

