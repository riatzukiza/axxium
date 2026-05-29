{
  description = "OpenCode development flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      systems = [
        "aarch64-linux"
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      forEachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      rev = self.shortRev or self.dirtyShortRev or "dirty";
    in
    {
      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            bun
            REDACTED_SECRETjs_20
            pkg-config
            openssl
            git
          ];
        };
      });

      overlays = {
        default =
          final: _prev:
          let
            REDACTED_SECRET_modules = final.callPackage ./nix/REDACTED_SECRET_modules.nix {
              inherit rev;
            };
            opencode = final.callPackage ./nix/opencode.nix {
              inherit REDACTED_SECRET_modules;
            };
            desktop = final.callPackage ./nix/desktop.nix {
              inherit opencode;
            };
          in
          {
            inherit opencode;
            opencode-desktop = desktop;
          };
      };

      packages = forEachSystem (
        pkgs:
        let
          REDACTED_SECRET_modules = pkgs.callPackage ./nix/REDACTED_SECRET_modules.nix {
            inherit rev;
          };
          opencode = pkgs.callPackage ./nix/opencode.nix {
            inherit REDACTED_SECRET_modules;
          };
          desktop = pkgs.callPackage ./nix/desktop.nix {
            inherit opencode;
          };
        in
        {
          default = opencode;
          inherit opencode desktop;
          # Updater derivation with fakeHash - build fails and reveals correct hash
          REDACTED_SECRET_modules_updater = REDACTED_SECRET_modules.override {
            hash = pkgs.lib.fakeHash;
          };
        }
      );
    };
}
