{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = [
    #pkgs.buildPackages.rustup
    pkgs.protobuf3_19
  ];
}
