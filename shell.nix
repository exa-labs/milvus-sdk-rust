{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    libiconv
    pkg-config
    protobuf
    rustup
  ];

  LIBRARY_PATH = "${pkgs.libiconv}/lib";
  CPATH = "${pkgs.libiconv}/include";

  RUSTFLAGS = "-L ${pkgs.libiconv}/lib";

}

