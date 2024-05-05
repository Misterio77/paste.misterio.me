{
  description = "Paste service and companion CLI tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    systems.url = "github:nix-systems/default";
  };

  outputs = { self, nixpkgs, systems }:
    let
      forAllSystems = nixpkgs.lib.genAttrs (import systems);
      pkgsFor = nixpkgs.legacyPackages;
    in
    rec {
      nixosModules.server = import ./server/nixos-module.nix;
      homeManagerModules.cli = import ./cli/hm-module.nix;

      packages = forAllSystems (system: {
        server = pkgsFor.${system}.callPackage ./server { };
        cli = pkgsFor.${system}.callPackage ./cli { };
        tests = pkgsFor.${system}.callPackage ./tests { };
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });

      hydraJobs = packages;
    };
}

