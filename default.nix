{ system ? builtins.currentSystem }:

let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  senpy-api = import ./senpy-api.nix { inherit sources pkgs; };

  name = "senpy-club/api";
  tag = "latest";
in pkgs.dockerTools.buildLayeredImage {
  inherit name tag;
  contents = [ senpy-api ];

  config = {
    Cmd = [ "/bin/senpy-api" ];
    Env = [ ];
    WorkingDir = "/";
  };
}
