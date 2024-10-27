{
  description = "A CLI tool to add logos to images";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource ./.;

        # Build dependencies
        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
          buildInputs = [
            pkgs.pkg-config
          ] ++ commonBuildInputs;
        };

        commonBuildInputs = with pkgs; [
          # Add any native build inputs here
          pkg-config
        ];

        commonRuntimeInputs = with pkgs; [
          # Add any runtime dependencies here
        ];

        logo-adder = craneLib.buildPackage {
          inherit src cargoArtifacts;
          buildInputs = commonBuildInputs;

          # Additional native build inputs if needed
          nativeBuildInputs = with pkgs; [];
        };
      in
      {
        checks = {
          inherit logo-adder;
        };

        packages.default = logo-adder;

        apps.default = flake-utils.lib.mkApp {
          drv = logo-adder;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ logo-adder ];
          buildInputs = commonBuildInputs ++ commonRuntimeInputs ++ [
            rustToolchain
            pkgs.rust-analyzer
            pkgs.cargo-watch
            pkgs.cargo-audit
            pkgs.cargo-outdated
          ];
        };
      });
}
