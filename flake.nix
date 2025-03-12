{
  description = "An anyrun plugin to search the web.";

  inputs.flake-parts.url = "github:hercules-ci/flake-parts";

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem = {pkgs, ...}: rec {
        packages = rec {
          default = anyrun-websearch-plus;
          anyrun-websearch-plus = pkgs.callPackage ./nix {};
        };

        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              cargo
              clippy
              rustc
              git
              rustfmt
              rust-analyzer
            ];

            inputsFrom = [packages.default];
          };
      };
    };
}
