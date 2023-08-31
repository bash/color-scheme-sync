{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/23.05";
    crane = {
      url = "github:ipetkov/crane/v0.13.0";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
      craneLib = crane.lib.${system};
    in {
      packages.${system}.default =
        craneLib.buildPackage {
          nativeBuildInputs = with pkgs; [
            pkg-config
            glib
          ];
          src = self;
        };
      nixosModules.default = {
        config = {
          systemd.user.services.color-scheme-sync = {
            enable = true;
            description = "Synchronizes the legacy gtk-theme preference with the new color scheme preference";
            unitConfig = {
              PartOf = "graphical-session.target";
              After = "graphical-session.target";
            };
            serviceConfig = {
              ExecStart = "${self.packages.${system}.default}/bin/color-scheme-sync";
              Restart = "always";
              RestartSec = 10;
              Slice = "session.slice";
            };
            wantedBy = [ "graphical-session.target" ];
          };
        };
      };
    };
}
