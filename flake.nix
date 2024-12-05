{
  description = "portfolio.rs nix shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs
    , rust-overlay
    , flake-utils
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustOverlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
        rustVersion = pkgs.rust-bin.nightly.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
        cargo-leptos = rustPlatform.buildRustPackage rec {
          pname = "cargo-leptos";
          version = "v0.2.22";
          src = pkgs.fetchFromGitHub {
            owner = "leptos-rs";
            repo = pname;
            rev = version;
            hash = "sha256-QPCYKlbPHuSBmwfkKdYhcVF81Lnirf65IYao7UVxz9Q=";
          };
          cargoHash = "sha256-SiMOIZ4rt1Vg59cDiGwoovmY8dCuYXGoviHQfLoyk28=";
          doCheck = false;
        };
      in
      with pkgs; {
        devShells.default = mkShell {
          buildInputs =
            [
              leptosfmt
              trunk
              cargo-leptos
              (rust-bin.selectLatestNightlyWith (toolchain:
                toolchain.default.override {
                  extensions = [ "rust-src" "rust-analyzer" ];
                  targets = [ "wasm32-unknown-unknown" ];
                }))
            ]
            ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

          shellHook = ''
            $SHELL
          '';
        };
      }
    );
}
