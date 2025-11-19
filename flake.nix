{
  description = "C# Dev Enviroment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    devShells.${system} = {
      rust = pkgs.mkShell {
        buildInputs = [
          pkgs.rustup
        ];
      };
      c = pkgs.mkShell {
        buildInputs = [
          pkgs.clang-tools
          pkgs.gcc
        ];
      };

      DOTNET_CLI_TELEMETRY_OPTOUT = "1";
    };
  };
}
