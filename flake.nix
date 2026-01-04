{
  description = "A Rust project for real for sensible centering in niri";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "better-focus";
          version = "0.1.0";

          src = ./.;

          cargoHash = "sha256-mBcJ0XNW8NtLf7HXrF/WImALLZYhOEiO9ba3HIfS/ek=";

          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [ pkgs.cargo pkgs.rustc pkgs.rust-analyzer ];
        };
      });
}
