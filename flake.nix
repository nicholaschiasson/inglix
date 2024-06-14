{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    legacy.url = "path:./legacy";
  };

  outputs = { self, nixpkgs, flake-utils, legacy }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      legacyShell = legacy.devShells.${system};
    in {
      devShells.default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo-watch
          pkgs.iconv
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
      devShells.legacy = legacyShell.default;
    });
}
