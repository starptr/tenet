{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
          # rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        let
          nativeBuildInputs = [ pkgs.pkg-config ]; # include rustToolchain in list if appropriate
          buildInputs = [ pkgs.openssl pkgs.rust-bin.stable.latest.default ];
        in
        {
          devShells.default = pkgs.mkShell {
            inherit buildInputs nativeBuildInputs;
          };
        }
      );
}
