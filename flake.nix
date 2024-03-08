{
  description = "Autovirt";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      version = "0.1.0";
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor =
        forAllSystems (system: import nixpkgs { inherit system; });
    in {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
          autovirt = pkgs.rustPlatform.buildRustPackage rec {
            pname = "autovirt";
            version = "0";
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
          };

          env = builtins.readFile ./.env;
        in {
          inherit autovirt;
          default = pkgs.dockerTools.buildImage {
            name = "autovirt";
            tag = "latest";
            copyToRoot = pkgs.buildEnv {
              name = "autovirt-root";
              paths = [ autovirt pkgs.cacert ];
              pathsToLink = [ "/bin" "/etc" ];
            };
            config = {
              Env = pkgs.lib.splitString "\n" env;
              Cmd = [ "/bin/autovirt" ];
              ExposedPorts = { "5432" = { }; };
            };
          };
        });
    };
}
