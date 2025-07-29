{
  description = "A development environment for this Rust project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.git-hooks-nix.flakeModule
      ];
      perSystem =
        {
          pkgs,
          self',
          config,
          ...
        }:
        {
          formatter = pkgs.nixfmt-rfc-style;
          packages = {
            default = self'.packages.sonare;
            sonare = pkgs.rustPlatform.buildRustPackage (_finalAttrs: {
              pname = "sonare";
              version = "0.1.0";
              src = ./.;
              buildInputs =
                with pkgs;
                [ openssl ]
                ++ (with pkgs.gst_all_1; [
                  glib-networking
                  gstreamer
                  gst-plugins-bad
                  gst-plugins-base
                  gst-plugins-good
                  gst-plugins-ugly
                  gst-plugins-rs
                  gst-libav
                  gst-vaapi
                  gst-devtools
                ]);
              nativeBuildInputs = with pkgs; [ pkg-config ];
              strictDeps = true;
              useFetchCargoVendor = true;
              cargoLock = {
                lockFile = ./Cargo.lock;
              };
            });
          };
          devShells = {
            default = self'.devShells.devel;
            devel = pkgs.mkShell {
              inputsFrom = [ self'.packages.sonare ];
              packages = with pkgs; [
                just # Make replacement
                vale # Markdown linter
                rust-analyzer # Rust lsp
                rustfmt # Rust formatter

                # Rust utils
                bacon # Watcher
                cargo-info
                hyperfine
                lldb # Debugger
                mprocs
              ];
              shellHook = ''
                ${config.pre-commit.installationScript}
              '';
            };
          };
          pre-commit.settings.hooks = {
            treefmt = {
              enable = true;
              settings = {
                formatters = with pkgs; [
                  nixfmt-rfc-style
                  rustfmt
                  yamlfmt
                ];
              };
            };
            deadnix = {
              enable = true;
              settings = {
                edit = true;
                quiet = true;
              };
            };
            #clippy.enable = true;
          };
        };
    };
}
