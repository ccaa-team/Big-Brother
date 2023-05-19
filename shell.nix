with import <nixpkgs> {};

mkShell {
  buildInputs = [ cargo-cross pkg-config openssl ];
}
