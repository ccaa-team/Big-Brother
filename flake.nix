{
  description = "Autovirt";

  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
    let 
      pkgs = import nixpkgs {
        inherit system;
      };
    in {
    devShell = pkgs.mkShell {
      DATABASE_URL="postgresql:///autovirt";
      buildInputs = with pkgs; [
        pkg-config
        openssl
        sqlx-cli
        cmake
      ];
    };
  });
}
