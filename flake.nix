{
  description = "Paste service and companion CLI tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
    in
    rec {
      nixosModules.server = import ./nix/server-module.nix;
      homeManagerModules.cli = import ./nix/cli-module.nix;

      packages = forAllSystems (system: {
        server = pkgsFor.${system}.callPackage ./nix/server.nix { };
        cli = pkgsFor.${system}.callPackage ./nix/cli.nix { };
        tests = pkgsFor.${system}.callPackage ./nix/tests.nix { };
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./nix/shell.nix { };
      });

      hydraJobs = packages;
    };
}

