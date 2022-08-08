{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in rec {
        # `nix build`
        packages.gitlab-rust-estimation-getter = naersk-lib.buildPackage {
          pname = "gitlab-rust-estimation-getter";
          root = ./.;
        };
        defaultPackage = packages.gitlab-rust-estimation-getter;

        # `nix run`
        apps.rust-in-time = flake-utils.lib.mkApp {
          drv = packages.gitlab-rust-estimation-getter;
        };
        defaultApp = apps.gitlab-rust-estimation-getter;

        # `nix develop`
        devShell =
          pkgs.mkShell { nativeBuildInputs = with pkgs; [ rustc cargo ]; };
      });
}
