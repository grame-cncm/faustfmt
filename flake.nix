# flake.nix
{
  description = "Topiary Flake for dev shell";

  inputs = {
    # Nixpkgs provides a vast collection of packages
    nixpkgs.url = "github:NixOS/nixpkgs/25.05"; # Use a specific release or branch if needed

    # Fenix provides up-to-date Rust toolchains
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs"; # Ensure Fenix uses the same nixpkgs
    };

    # flake-utils simplifies writing flakes for multiple systems
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      fenix,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        f = fenix.packages.${system};
      in
      {
        # Define a development shell with the specified tools
        devShells.default = pkgs.mkShell {
          name = "rust-dev";

          packages = with pkgs; [
            # Stable Rust toolchain from Fenix
            f.stable.toolchain # Use f.latest.toolchain for nightly

            # rust-analyzer for editor integration
            f.rust-analyzer

            # rust-src, rustfmt, and clippy are included in the toolchain
            # No need to add them separately
          ];

          # Optionally set environment variables
          RUST_BACKTRACE = 1;

          # Add any other build inputs your project needs (e.g., wasm-pack, nodejs)
          # Example:
          # wasm-pack
          # nodejs
        };
      }
    );
}
