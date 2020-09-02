let
  sources = import ./nix/sources.nix;
  pkgs = import sources.nixpkgs { };
  frameworks = pkgs.darwin.apple_sdk.frameworks;
in
pkgs.mkShell {
  buildInputs = [
    pkgs.cargo
  ];

  propagatedBuildInputs = with pkgs; [
    frameworks.Security
  ];

   NIX_LDFLAGS = "-F${frameworks.Security}/Library/Frameworks -framework Security";
}
