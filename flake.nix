{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs = { nixpkgs, crane, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; }; in
      let
        craneLib = crane.lib.${system}.overrideToolchain 
          fenix.packages.${system}.latest.toolchain;
      in
      let workspace = (fromTOML (builtins.readFile ./Cargo.toml)).workspace; 
          pkgToml = fromTOML (builtins.readFile ./fn_activator/Cargo.toml); 
          package = craneLib.buildPackage {
            src = craneLib.cleanCargoSource (craneLib.path ./.);
            pname = pkgToml.package.name;
            version = workspace.package.version;
            doCheck = false;
            nativeBuildInputs = [pkgs.pkg-config pkgs.udev];
          }; 
      in
    {
      packages.default = package;
      nixosModules.default = {config, ...}: {
        options = {
          services.udev.extraRules = "ACTION==\"add\", KERNEL==\"hidraw[0-9]*\", RUN+=\"${package}/bin/fn_activator\"";
        };
        config = {};
      };
    });
}
