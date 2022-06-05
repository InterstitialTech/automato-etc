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
              rustc
            ];
          };
        }
    );
}

