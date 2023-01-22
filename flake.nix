{
  description = "rust dev environment";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pname = "rust-dev";
        pkgs = nixpkgs.legacyPackages."${system}";
      in
        rec {
          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo
              cargo-watch
              cargo-edit
              rustc
              rustfmt
              rust-analyzer
              openssl.dev
              udev
              pkgconfig
              elm2nix
              elmPackages.elm
              elmPackages.elm-analyse
              elmPackages.elm-doc-preview
              elmPackages.elm-format
              elmPackages.elm-live
              elmPackages.elm-test
              elmPackages.elm-upgrade
              elmPackages.elm-xref
              elmPackages.elm-language-server
              elmPackages.elm-verify-examples
              elmPackages.elmi-to-json
              elmPackages.elm-optimize-level-2
            ];
          };
        }
    );
}

