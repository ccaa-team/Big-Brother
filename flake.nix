{
  description = "Autovirt";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane }:
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
          inherit (pkgs) lib;
          craneLib = crane.lib.${system};
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          commonArgs = {
            inherit src;
            strictDeps = true;
          };
          artifacts = craneLib.buildDepsOnly commonArgs;
          autovirt = craneLib.buildPackage (commonArgs // {
            inherit artifacts;
          });
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
