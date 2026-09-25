# SPDX-FileCopyrightText: 2026 Meowdia Community
# SPDX-License-Identifier: MIT OR Apache-2.0

{
  description = "IANA registry types and snapshot tooling";

  inputs = {
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.11";
  };

  outputs =
    {
      self,
      crane,
      fenix,
      flake-utils,
      nixpkgs,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (
          pkgs.fenix.stable.withComponents [
            "cargo"
            "rustc"
            "rustfmt"
            "clippy"
          ]
        );

        commonArgs = {
          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.unions [
              (craneLib.fileset.commonCargoSources ./.)
              ./iana
            ];
          };
          strictDeps = true;
          cargoExtraArgs = "--workspace";
        };

        cargoArtifacts = craneLib.buildDepsOnly (
          commonArgs
          // {
            cargoExtraArgs = "--workspace";
            pname = "iana";
          }
        );

        cargoArtifactsDev = cargoArtifacts.overrideAttrs (
          final: prev: {
            CARGO_PROFILE = "dev";
          }
        );

        ianaClippy = craneLib.cargoClippy (
          commonArgs
          // {
            CARGO_PROFILE = "dev";
            cargoArtifacts = cargoArtifactsDev;
            cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
          }
        );

        ianaFmt = craneLib.cargoFmt (commonArgs // { cargoExtraArgs = "--all"; });

        ianaTest = craneLib.cargoTest (
          commonArgs
          // {
            CARGO_PROFILE = "dev";
            cargoArtifacts = cargoArtifactsDev;
            cargoTestExtraArgs = "--all-features";
          }
        );

        ianaLib = craneLib.cargoBuild (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );

        ianaFeatureTest = craneLib.cargoTest (
          commonArgs
          // {
            CARGO_PROFILE = "dev";
            cargoArtifacts = cargoArtifactsDev;
            cargoTestExtraArgs = "--no-default-features --features sdp-parameters,metadata";
          }
        );

        iana = craneLib.mkCargoDerivation (commonArgs // {
          pname = "xtask";
          cargoArtifacts = cargoArtifactsDev;
          CARGO_PROFILE = "dev";

          nativeBuildInputs = [ pkgs.git ];

          buildPhaseCargoCommand = "cargo run -p xtask -- iana check";
        });
      in
      {
        checks = {
          clippy = ianaClippy;
          test = ianaTest;
          features = ianaFeatureTest;
          fmt = ianaFmt;
          iana = iana;
        };

        apps = builtins.listToAttrs (
          builtins.map
            (
              name:
              let
                cmd = pkgs.writeShellScript "just-${name}" "${pkgs.just}/bin/just ${name}";
              in
              {
                inherit name;
                value = {
                  type = "app";
                  program = "${cmd}";
                  meta = {
                    description = "runs `just ${name}`";
                  };
                };
              }
            )
            [
              "build"
              "test"
              "lint"
              "check"
              "clean"
              "fmt"
            ]
        );

        packages = {
          ci_clippy = ianaClippy;
          ci_test = ianaTest;
          ci_test_features = ianaFeatureTest;
          ci_fmt = ianaFmt;
          deps = cargoArtifacts;
          deps_dev = cargoArtifactsDev;
          iana_check = iana;
          lib = ianaLib;
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            pkgs.fenix.stable.toolchain
            just
            reuse
          ];
        };
      }
    );
}
