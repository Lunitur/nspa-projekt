{
  description = "A basic Rust project with full tooling";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        tex = (pkgs.texlive.combine {
          inherit (pkgs.texlive)
            scheme-medium
            titlesec
            fancyhdr
            background
            microtype
            ulem
            textpos
            everypage
            etextools
            environ
            fmtcount
            koma-script
            babel
            babel-croatian
            datetime
            geometry
            amsfonts
            csquotes
            tcolorbox
            pgf
            pgfplots
            arydshln
            float
            xcolor
            breqn
            thmtools
            multirow
            hyperref
            booktabs
            listings
            letltxmacro
            adjustbox
            enumitem
            biblatex
            placeins
            mathtools
            autonum
            url;
        });
      in
      {
        packages.default = rustPlatform.buildRustPackage {
          pname = "nspa-projekt";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ ];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            rustToolchain
            pkgs.cargo-edit
            pkgs.cargo-watch
            pkgs.bacon
            pkgs.pkg-config
            tex
            pkgs.bibtex-tidy
            pkgs.texlab
            pkgs.texmaker
          ];

          buildInputs = [ ];

          shellHook = ''
            echo "Rust and LaTeX development environment loaded"
            rustc --version
            cargo --version
            latex --version
          '';
        };
      }
    );
}
