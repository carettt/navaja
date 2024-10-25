{
  description = "rust project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
        nativeBuildInputs = [
          pkgs.cargo
          pkgs.rustc
          pkgs.pkg-config
          pkgs.openssl
        ];

        shellHook = ''
          ${pkgs.cowsay}/bin/cowsay "entered dev env!" | ${pkgs.lolcat}/bin/lolcat -F 0.5
        '';
        };
      });
}
