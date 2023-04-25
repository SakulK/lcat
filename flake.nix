{
  nixConfig = {
    substituters =
      [ "https://cache.nixos.org" "https://sakulk-lcat.cachix.org" ];

    trusted-public-keys = [
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "sakulk-lcat.cachix.org-1:BWNQQg21NQR+iBJzun14S76A3px2QJJb67hI3QEqUX0="
    ];
  };

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) { inherit system; };

        naersk' = pkgs.callPackage naersk { };

      in {
        # For `nix build` & `nix run`:
        packages.default = naersk'.buildPackage { src = ./.; };

        # For `nix develop`:
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustc
            cargo
            gcc
            rust-analyzer
            clippy
            rustfmt
          ];
        };
      });
}
