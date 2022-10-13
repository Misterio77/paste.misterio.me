{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.programs.pmis;

in {
  options.programs.pmis = {
    enable = mkEnableOption "pmis, a CLI for paste.misterio.me";

    package = mkOption {
      type = types.package;
      default = pkgs.pmis;
      defaultText = literalExpression "pkgs.pmis";
      description = "Package providing <command>pmis</command>.";
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];
  };
}
