{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };
    utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      rust-overlay,
    }:
    utils.lib.eachSystem [ "aarch64-linux" "x86_64-linux" ] (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # Build inputs
        requiredPrograms = with pkgs; [
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          trunk
          pkg-config
        ];

        # Compiletime/Runtime deps (native linux build)
        requiredLibsLinux = with pkgs; [
          # misc. libraries
          openssl

          # GUI libs
          libxkbcommon
          libGL
          fontconfig

          # wayland libraries
          wayland

          # x11 libraries
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
        ];

        # IDE/shell dependencies
        developPrograms = with pkgs; [
          clippy
          rust-analyzer-unwrapped
          cargo-edit
        ];

        mkLinuxLdLibraryPathExport = libs: ''
          FLAKE_LIBDIR="${pkgs.lib.makeLibraryPath libs}"
          RUST_LIBDIR=$(rustc --print target-libdir)
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$FLAKE_LIBDIR:$RUST_LIBDIR:target/debug/deps:target/debug:${pkgs.stdenv.cc.cc.lib}/lib"
        '';
      in
      {
        # `nix develop`
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = developPrograms ++ requiredPrograms;
          buildInputs = requiredLibsLinux;

          shellHook = ''
            ${mkLinuxLdLibraryPathExport buildInputs}
          '';
        };
      }
    );
}
