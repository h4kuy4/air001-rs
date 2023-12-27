{
    description = "A Nix-flake-based development environment for Kage tunnel";

    inputs = {
        nixpkgs.url      = "github:NixOS/nixpkgs/nixos-23.11";
        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = { self, nixpkgs, rust-overlay, ... }: let
        supportedSystems = [
            "x86_64-linux"
            "aarch64-linux"
            "aarch64-darwin"
        ];

        forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
            pkgs = import nixpkgs { 
                inherit system; 
                overlays = [ 
                    (import rust-overlay)
                ];
            };
        });
    in {
        devShells = forAllSystems ({pkgs}: {
            default = pkgs.mkShell {
                packages = (with pkgs; [
                    openssl
                    pkg-config
                    rust-analyzer
                    gcc-arm-embedded
                    openocd
                    (probe-rs.override {
                        rustPlatform.buildRustPackage = 
                            args: rustPlatform.buildRustPackage (args // rec {
                                src = fetchFromGitHub {
                                    owner = "LittleJianCH";
                                    repo = "probe-rs";
                                    rev = "new_target/AirMCU";
                                    hash = "sha256-z/hoPWrxyoW30gdzWanelVNiA8E7CALEsYXlYKS0QDE=";
                                };

                                cargoHash = "sha256-2JTa3hxBVB1PNzaKOSPhm5Sj41jU88JrpeJSx5N0nC4=";
                            });
                    })

                    (rust-bin.stable.latest.default.override {
                        targets = [ "thumbv6m-none-eabi" ];
                    })
                ]);

                shellHook = ''
                    exec fish
                '';
            };
        });
    };
}
