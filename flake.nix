{
  description = "Autovirt";

  inputs = { nixpkgs.url = "nixpkgs"; };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      version = "0.1.0";
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
          autovirt = pkgs.rustPlatform.buildRustPackage {
            pname = "autovirt";
            inherit version;
            src = ./.;

            #cargoSha256 = pkgs.lib.fakeSha256;
            cargoSha256 = "sha256-r7TCEgbiDhRoX/BH+PicAhay36ecy42gEP0ezrYEw90=";
          };
          env = builtins.readFile ./.env;
        in { inherit autovirt;
        default = pkgs.dockerTools.buildImage {
          name = "autovirt";
          contents = [ autovirt pkgs.cacert ];
          config = {
            Env = pkgs.lib.splitString "\n" env;
            Cmd = [ "${autovirt}/bin/autovirt" ];
          };
        });
    };
}
