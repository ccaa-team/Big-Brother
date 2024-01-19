{
  description = "Autovirt";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  outputs = { self, nixpkgs, rust-overlay }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      version = "0.1.0";
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      overlays = [ (import rust-overlay) ];
      nixpkgsFor =
        forAllSystems (system: import nixpkgs { inherit system overlays; });
    in {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
          autovirt = pkgs.rustPlatform.buildRustPackage {
            pname = "autovirt";
            inherit version;
            src = ./.;

            #cargoSha256 = pkgs.lib.fakeSha256;
            cargoSha256 = "sha256-U/vzbNAAQvlBF9UZTmZ5/t7tUPHE3Hnc/ZkW8eQoAac=";
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
      devShells = forAllSystems (system:
        let pkgs = nixpkgsFor.${system};
        in {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [rust-bin.stable.latest.default];
          };
        });
    };
}
