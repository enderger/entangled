{
  description = "My entry for the 2023 GitHub Game Off";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, utils, ... }@inputs:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs { inherit system; config.allowUnfree = true; };
      fenix = inputs.fenix.packages."${system}";
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          (fenix.combine (with fenix; [
            default.rustc default.cargo default.clippy complete.rust-src
            targets.wasm32-unknown-unknown.latest.rust-std
            rust-analyzer
          ]))
          pkg-config just
          udev alsaLib lutris
          xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi
          vulkan-tools vulkan-headers vulkan-loader vulkan-validation-layers mesa
          act
        ];
        shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
          pkgs.lib.makeLibraryPath (with pkgs; [
            udev
            alsaLib
            vulkan-loader
          ])};
        export RUST_SRC_PATH = "${fenix.complete.rust-src}/lib/rustlib/src/rust/library"
        "'';
      };
    });
}
