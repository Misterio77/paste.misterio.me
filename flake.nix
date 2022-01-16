{
  description = "Paste service";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    let
      name = "paste-misterio-me";
      overlay = final: prev: {
        ${name} = final.callPackage ./default.nix { };
      };
      overlays = [ overlay ];
    in
    rec {
      inherit overlay overlays;

      nixosModules."${name}" = import ./module.nix {};
      nixosModule = nixosModules."${name}";
    } //
    (utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system overlays; };
      in
      rec {
        # nix build
        packages.${name} = pkgs.${name};
        defaultPackage = packages.${name};

        # nix run
        apps.${name} = utils.lib.mkApp { drv = packages.${name}; };
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

