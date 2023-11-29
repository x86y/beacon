{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        buildInputs = with pkgs; [
          cbqn
          xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
        ];
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

        beacon = crane.lib.${system}.buildPackage {
          inherit buildInputs;
          src = pkgs.lib.cleanSourceWith {
            src    = ./.;
            filter = path: type:
              (pkgs.lib.hasInfix "/src/" path)
              || (crane.lib.${system}.filterCargoSources path type);
          };
        };
      in
        with pkgs; {

          # nix build, nix run, …
          packages = {
            inherit beacon;
            default = pkgs.symlinkJoin {
              name = "beacon";
              paths = [ beacon ];
              buildInputs = [ pkgs.makeWrapper ];
              postBuild = ''
                wrapProgram $out/bin/beacon \
                  --prefix LD_LIBRARY_PATH : "${LD_LIBRARY_PATH}"
              '';
              cargoExtraArgs = "--release";
            };
          };

          # nix develop
          devShells.default = mkShell {
            inherit LD_LIBRARY_PATH;
            buildInputs = buildInputs ++ [ cargo rustc rust-analyzer ];
          };
        }
    );
}
