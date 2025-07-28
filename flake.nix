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
    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, crane }:
    utils.lib.eachSystem ["aarch64-linux" "x86_64-linux"] (system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs { inherit system overlays; };
        craneLib = crane.mkLib pkgs;

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

        # Autofetch project info from Cargo
        cargoDesc = pkgs.lib.trivial.importTOML ./Cargo.toml;
        projectName = cargoDesc.package.name;
        projectVersion = cargoDesc.package.version;

        packageDef = rec {
          pname = projectName;
          version = projectVersion;

          src =
            pkgs.lib.cleanSourceWith {
              src = ./.;
              filter = path: type: craneLib.filterCargoSources path type;
            };

          # https://github.com/NixOS/nix/issues/4623
          # GIT_LFS_SKIP_SMUDGE = 1;
          strictDeps = true;
          nativeBuildInputs = requiredPrograms;
        };
        mkLinuxLdLibraryPathExport = libs: ''
          FLAKE_LIBDIR="${pkgs.lib.makeLibraryPath libs}"
          RUST_LIBDIR=$(rustc --print target-libdir)
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$FLAKE_LIBDIR:$RUST_LIBDIR:target/debug/deps:target/debug:${pkgs.stdenv.cc.cc.lib}/lib"
        '';
      in
      rec {
        # `nix develop`
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = developPrograms ++ requiredPrograms;
          buildInputs = requiredLibsLinux;

          shellHook = ''
            ${mkLinuxLdLibraryPathExport buildInputs}
          '';
        };
      });
}
