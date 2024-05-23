{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    rust-flake.url = "github:juspay/rust-flake";
    rust-flake.inputs.nixpkgs.follows = "nixpkgs";

    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
    just-flake.url = "github:juspay/just-flake";
    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.just-flake.flakeModule
        inputs.pre-commit-hooks-nix.flakeModule
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];
      perSystem = { config, self', pkgs, lib, system, ... }: {
        rust-project.crane.args = {
          nativeBuildInputs = [ ];
        };

        just-flake.features = {
          treefmt.enable = true;
          rust.enable = true;
          convco.enable = true;
        };

        treefmt.config = {
          projectRootFile = "flake.nix";
          programs = {
            nixpkgs-fmt.enable = true;
            rustfmt.enable = true;
          };
        };

        pre-commit = {
          check.enable = true;
          settings = {
            hooks = {
              treefmt.enable = true;
              convco.enable = true;
            };
          };
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [
            self'.devShells.search
            config.treefmt.build.devShell
            config.just-flake.outputs.devShell
            config.pre-commit.devShell
          ];
          packages = [
            pkgs.cargo-watch
            config.pre-commit.settings.tools.convco
            pkgs.cargo-nextest
            pkgs.bacon
          ];
        };
      };
    };
}
