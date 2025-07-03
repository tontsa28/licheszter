{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachSystem
      [
        "aarch64-linux"
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ]
      (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
        in
        {
          devShells.default = pkgs.mkShell {
            packages =
              with pkgs;
              [
                (rust-bin.stable.latest.default.override {
                  extensions = [
                    "rust-src"
                    "rust-analyzer"
                    "clippy"
                  ];
                })
              ]
              ++ lib.optionals stdenv.isDarwin [
                libiconv
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];

            shellHook = ''
              echo "Welcome to the Rust development environment!"
              echo "Rust version: $(rustc --version)"
            '';
          };
        }
      );
}
