{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo-watch
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
          pkgs.hurl
          pkgs.iconv
          pkgs.just
          pkgs.lldb
          pkgs.nodePackages.typescript-language-server
          pkgs.nodePackages.vscode-langservers-extracted
          pkgs.rustup
          pkgs.starship
        ];
        shellHook = ''
          source .env
          rustup component add rust-analyzer clippy
          eval "$(starship init bash)"
        '';
      };
    });
}
