{
  description = "Realtime Vulkan ray tracing";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; 
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay,... }: let
    lib = {
      inherit (flake-utils.lib) defaultSystems eachSystem;
    };
    supportedSystems = [ "x86_64-linux" ];
  in lib.eachSystem supportedSystems (system: let
    nightlyVersion = "2024-03-15";

    #pkgs = mars-std.legacyPackages.${system};
    pkgs = import nixpkgs {
        inherit system;
        overlays = [
          (import rust-overlay)
          #(import ./pkgs)
        ];
      };
    pinnedRust = pkgs.rust-bin.nightly.${nightlyVersion}.default.override {
      extensions = ["rustc-dev" "rust-src" "rust-analyzer-preview" "llvm-tools"];
      targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown"];
    };
    rustPlatform = pkgs.makeRustPlatform {
      rustc = pinnedRust;
      cargo = pinnedRust;
    };
    cargoExpand = pkgs.cargo-expand.override { inherit rustPlatform; };
  in {
    
devShell = pkgs.mkShell rec {
  hardeningDisable = [
    "fortify"
  ];
  nativeBuildInputs = [
    pinnedRust 
    # cargoExpand
  ];
  buildInputs = with pkgs; [
    sass
    cargo-leptos
    # llvmPackages_17.libllvm
#    alsaLib
#    binaryen
#    fontconfig
#    freetype
#    libxkbcommon
#    spirv-tools
    #udev

#    xorg.libXi
#    xorg.libXrandr
#    gcc-unwrapped.lib
  ];


  shellHook = ''
  '';
};

  });
}
