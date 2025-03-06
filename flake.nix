{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        nativeBuildInputs = with pkgs; [
          pkg-config
          rustToolchain
        ];
        buildInputs = with pkgs; [
          cargo-watch
          docker-compose-language-service
          dockerfile-language-server-nodejs
          hurl
          iconv
          just
          lldb
          nil
          nixfmt-rfc-style
          nodePackages.typescript-language-server
          nodePackages.vscode-langservers-extracted
          rust-analyzer
          starship
          yaml-language-server
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell {
          inherit buildInputs nativeBuildInputs;
          shellHook = ''
            source .env
            rustc --version
            eval "$(starship init bash)"
          '';
        };
        packages.default =
          let
            manifest = (pkgs.lib.importTOML ./services/entrypoint/Cargo.toml).package;
          in
          (pkgs.makeRustPlatform {
            rustc = rustToolchain;
            cargo = rustToolchain;
          }).buildRustPackage
            {
              pname = manifest.default-run;
              version = self.shortRev or self.dirtyShortRev;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
            };
      }
    );
}
