{
  description = "Paste service and companion CLI tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
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

