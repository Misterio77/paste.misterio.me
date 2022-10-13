{
  description = "Paste service and companion CLI tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
    in
    rec {
      nixosModules.server = import ./server/module.nix;
      homeManagerModules.cli = import ./cli/module.nix;

      packages = forAllSystems (system: rec {
        default = server;
        server = pkgsFor.${system}.callPackage ./server { };
        cli = pkgsFor.${system}.callPackage ./cli { };
      });

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix { };
      });
      hydraJobs = packages;
    };
}

