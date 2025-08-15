{
  description = "sway-focus-flash: brief opacity animation for newly focused windows in Sway";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
    ];
    eachSystem = f: builtins.listToAttrs (map (system: {
      name = system;
      value = f system;
    }) systems);
  in {
    packages = eachSystem (system: let
      pkgs = import nixpkgs { inherit system; };
      lib = pkgs.lib;
    in {
      sway-focus-flash = pkgs.rustPlatform.buildRustPackage rec {
        pname = "sway-focus-flash";
        version = "0.1.0"; 
        src = self;

        cargoLock = {
          lockFile = self + "/Cargo.lock";
        };

        doCheck = false;

        meta = with lib; {
          description = "Small Sway utility that briefly animates opacity of the newly focused window";
          mainProgram = "sway-focus-flash";
          platforms = platforms.linux;
        };
      };

      default = self.packages.${system}.sway-focus-flash;
    });

    apps = eachSystem (system: {
      default = {
        type = "app";
        program = "${self.packages.${system}.sway-focus-flash}/bin/sway-focus-flash";
      };
    });

    overlays.default = final: prev: let
      lib = final.lib;
    in {
      sway-focus-flash = final.rustPlatform.buildRustPackage rec {
        pname = "sway-focus-flash";
        version = "0.1.0";
        src = self;
        cargoLock = { lockFile = self + "/Cargo.lock"; };
        doCheck = false;
        meta = with lib; {
          description = "Small Sway utility that briefly animates opacity of the newly focused window";
          mainProgram = "sway-focus-flash";
          platforms = platforms.linux;
        };
      };
    };
  };
} 