{ pkgs }: let
  nixos-lib = import "${pkgs.path}/nixos/lib" { };
  runTest = test: nixos-lib.runTest {
    hostPkgs = pkgs;
    imports = [ test ];
  };
  server = pkgs.callPackage ../server { };
  cli = pkgs.callPackage ../cli { };
in runTest {
  name = "default test";

  nodes.machine = {
    imports = [ ../server/nixos-module.nix ];
    services.paste-misterio-me = {
      enable = true;
      port = 8080;
      package = server;
      environmentFile = builtins.toFile "env" ''
        ROCKET_SECRET_KEY=Cb0uFIrmXCr4M+k4lKnohQc7vTMM0RmpxjsdnnKui1k=
      '';
    };
    environment = {
      systemPackages = [ cli ];
      sessionVariables.PMIS_API = "http://localhost:8080";
    };
  };

  testScript = ''
    machine.wait_for_unit("paste-misterio-me.service")
    # Run migrations
    machine.succeed("cat ${server}/etc/db/*.sql | sudo -u paste psql")
    machine.wait_until_succeeds("curl http://localhost:8080")
    # Try uploading and downloading a paste
    id = machine.succeed('echo foo-bar | pmis u | cut -d "/" -f5 ').strip()
    machine.succeed(f'pmis d {id} | grep foo-bar')
  '';
}
