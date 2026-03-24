{
  description = "Dev shell and package provider for this program.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        libs = with pkgs; [
        ];
      in
      {
        devShells.default = with pkgs; mkShell {
          nativeBuildInputs = [
            pkg-config
          ] ++ libs;
          buildInputs = [
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default))
          ];

          LD_LIBRARY_PATH = (pkgs.lib.makeSearchPath "lib" libs);
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

          shellHook = ''
            fish
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cailloux";
          version = "0.1.0";

          cargoHash = "sha256-QwQfqcyjovzWV4tdP4D5p5GkzfrQMSFQWLbtbXSfuT4=";

          src = pkgs.fetchFromGitHub {
            owner = "MaxUlysse3";
            repo = "cailloux";
            rev = "master";
            sha256 = "sha256-4Yn3eBCe9zI08telEHCrP8mHYc7PQ5sfoMbF40rMnaY=";
          };

          meta = {
            description = "A utility for my environement.";
            homepage = "https://github.com/MaxUlysse3/cailloux";
            license = "pkgs.lib.license.mit";
            mainProgram = "cailloux";
          };
        };
      }
    );
}
