{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        {
          pkgs,
          lib,
          system,
          ...
        }:
        let
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust;
          overlays = [ inputs.rust-overlay.overlays.default ];
          src = lib.cleanSource ./.;
          buildInputs = [
            pkgs.openssl
            pkgs.sqlite
          ];
          nativeBuildInputs = [
            # Build tools
            rust
            pkgs.pkg-config

            # LSP
            pkgs.nil
          ];
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src buildInputs nativeBuildInputs;
          };
          test-matrix-rust-sdk = craneLib.buildPackage {
            inherit
              src
              cargoArtifacts
              buildInputs
              nativeBuildInputs
              ;
            strictDeps = true;
            doCheck = true;

            installPhaseCommand = ''
              echo "actually installing contents of $postBuildInstallFromCargoBuildLogOut to $out"
              mkdir -p $out
              find "$postBuildInstallFromCargoBuildLogOut" -mindepth 1 -maxdepth 1 | xargs -r mv -t $out

              wrapProgram $out/bin/tenpo \
                --set LD_LIBRARY_PATH ${lib.makeLibraryPath buildInputs}
            '';

            meta = {
              licenses = [ lib.licenses.mit ];
              mainProgram = "tenpo";
            };
          };
          cargo-clippy = craneLib.cargoClippy {
            inherit
              src
              cargoArtifacts
              buildInputs
              nativeBuildInputs
              ;
            cargoClippyExtraArgs = "--verbose -- --deny warnings";
          };
          cargo-doc = craneLib.cargoDoc {
            inherit
              src
              cargoArtifacts
              buildInputs
              nativeBuildInputs
              ;
          };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system overlays;
          };

          treefmt = {
            projectRootFile = "flake.nix";

            # Nix
            programs.nixfmt.enable = true;

            # Rust
            programs.rustfmt.enable = true;
            settings.formatter.rustfmt.command = "${rust}/bin/rustfmt";

            # TOML
            programs.taplo.enable = true;

            # GitHub Actions
            programs.actionlint.enable = true;

            # Markdown
            programs.mdformat.enable = true;

            # ShellScript
            programs.shellcheck.enable = true;
            programs.shfmt.enable = true;
          };

          packages = {
            inherit test-matrix-rust-sdk;
            default = test-matrix-rust-sdk;
            doc = cargo-doc;
          };

          checks = {
            inherit cargo-clippy;
          };

          devShells.default = pkgs.mkShell {
            inherit buildInputs nativeBuildInputs;

            shellHook = ''
              export PS1="\n[nix-shell:\w]$ "
            '';
          };
        };
    };
}
