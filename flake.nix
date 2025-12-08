{
  description = "Advent of Code 2025 solutions";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        toolchain = fenix.packages.${system}.complete.toolchain;
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        all = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
          }
        );
      in
      {
        checks = {
          inherit all;
        };

        packages = {
          inherit all;
          default = all;
        };

        # apps = {
        #   my-cli = {
        #     type = "app";
        #     program = "${all}/bin/my-cli";
        #   };
        #   my-server = {
        #     type = "app";
        #     program = "${all}/bin/my-server";
        #   };
        # };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
        };
      }
    );
}
