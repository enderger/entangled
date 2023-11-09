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
            default.rustc default.cargo default.clippy
            rust-analyzer
          ]))
          pkg-config
          udev alsaLib lutris
          xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi
          vulkan-tools vulkan-headers vulkan-loader vulkan-validation-layers mesa
        ];
        shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
          pkgs.lib.makeLibraryPath (with pkgs; [
            udev
            alsaLib
            vulkan-loader
          ])
        }"'';
      };
    });
}
