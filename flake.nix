{
  description = "A devtools for coincli project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      perSystem = {
        self',
        pkgs,
        ...
      }: {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "coincli";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          meta.mainProgram = "coincli";
        };

        apps.default = {
          type = "app";
          program = pkgs.lib.getExe self'.packages.default;
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            rust-analyzer
            rustfmt
            clippy
          ];
        };

        checks = {
          formatting =
            pkgs.runCommand "coincli-fmt-check" {
              nativeBuildInputs = with pkgs; [
                cargo
                rustfmt
              ];
              src = ./.;
            } ''
              cp -r "$src" ./src
              chmod -R u+w ./src
              cd ./src
              cargo fmt --check
              touch "$out"
            '';

          clippy =
            pkgs.runCommand "coincli-clippy" {
              nativeBuildInputs = with pkgs; [
                cargo
                clippy
                rustc
              ];
              src = ./.;
            } ''
              cp -r "$src" ./src
              chmod -R u+w ./src
              cd ./src
              cargo clippy --offline --all-targets -- -D warnings
              touch "$out"
            '';
        };

        formatter = pkgs.nixfmt-rfc-style;
      };
    };
}
