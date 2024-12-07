{
  description = "aoc2024";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    naersk,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [fenix.overlays.default];
        };

        cargo-aoc = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-aoc";
          version = "0.3.7";

          src = pkgs.fetchFromGitHub {
            owner = "gobanos";
            repo = pname;
            rev = version;
            hash = "sha256-k9Lm91+Bk6EC8+KfEXhSs4ki385prZ6Vbs6W+18aZSI=";
          };
          cargoHash = "sha256-DKP9YMbVojK7w5pkX/gok4PG6WUjhqUdvTwSir05d0s=";
          doCheck = false;
        };

        toolchain = with fenix.packages.${system};
          combine [
            default.rustc
            default.cargo
            default.clippy
            default.rustfmt
          ];

        naersk' = naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        };

        buildInputs = with pkgs; [openssl cargo-aoc];
        shellPkgs = with pkgs; [
          cargo-aoc
          cargo-flamegraph
        ];
      in rec {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
          inherit buildInputs;
        };

        devShell = pkgs.mkShell {
          inputsFrom = [defaultPackage];
          packages = shellPkgs;
        };
      }
    );
}
