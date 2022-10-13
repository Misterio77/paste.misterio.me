{
  description = "Companion CLI tool for paste.misterio.me";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    let
      name = "pmis";
      overlay = final: prev: {
        ${name} = final.callPackage ./default.nix { };
      };
      overlays = [ overlay ];
    in
    rec {
      homeManagerModules.${name} = import ./module.nix;
      homeManagerModule = homeManagerModules.${name};
      inherit overlay overlays;
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
          buildInputs = with pkgs; [ rustc rust-analyzer rustfmt clippy ];
        };
      }));
}

