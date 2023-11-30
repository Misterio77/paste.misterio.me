{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.programs.pmis;

in {
  options.programs.pmis = {
    enable = mkEnableOption "pmis, a CLI for paste.misterio.me";

    package = mkOption {
      type = types.package;
      default = pkgs.callPackage ./. { };
      description = "Package providing <command>pmis</command>.";
    };

    apiUrl = mkOption {
      default = null;
      type = types.nullOr types.str;
      description = ''
        Default pmis API URL to use.
      '';
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];
    sessionVariables = {
      PMIS_API = lib.optionalString (cfg.apiUrl != null) cfg.apiUrl;
    };
  };
}
