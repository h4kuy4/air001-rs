{
    description = "A Nix-flake-based development environment for Kage tunnel";

    inputs = {
        nixpkgs.url      = "github:NixOS/nixpkgs/nixos-23.11";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { self, nixpkgs, rust-overlay, ... }: let
        system = "x86_64-linux";
    in {
        devShells."${system}" = {
            default = let
                pkgs = import nixpkgs {
                    inherit system;

                    overlays = [
                        (import rust-overlay) 
                    ];
                };
            in pkgs.mkShell {
                packages = with pkgs; [
                    openssl
                    pkg-config
                    rust-analyzer
                    gcc-arm-embedded
                    openocd

                    (rust-bin.stable.latest.default.override {
                        targets = [ "thumbv6m-none-eabi" ];
                    })
                ];

                shellHook = ''
                    exec fish
                '';
            };
        };
    };
}
