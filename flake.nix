{
  description = "Converts Bash aliases to Nushell - nu-alias-converter";

  inputs = {
    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1";
    fenix = {
      url = "https://flakehub.com/f/nix-community/fenix/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
    crane,
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forEachSupportedSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              self.overlays.default
            ];
          };
        });
  in {
    overlays.default = final: prev: {
      rustToolchain = with fenix.packages.${prev.stdenv.hostPlatform.system};
        combine (with stable; [
          clippy
          rustc
          cargo
          rustfmt
          rust-src
        ]);
    };

    devShells = forEachSupportedSystem ({pkgs}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          openssl
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          rust-analyzer
        ];

        env = {
          # Required by rust-analyzer
          RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
        };
      };
    });

    packages = forEachSupportedSystem ({pkgs}: let
      craneLib = (crane.mkLib pkgs).overrideToolchain pkgs.rustToolchain;
      commonArgs = {
        src = ./.;
        strictDeps = true;
        buildInputs = [pkgs.openssl];
        nativeBuildInputs = with pkgs; [pkg-config];
        meta = with pkgs.lib; {
          description = "Converts Bash aliases to Nushell";
          homepage = "https://github.com/marcelarie/nu-alias-converter";
          license = with licenses; [mit asl20];
          maintainers = [];
          platforms = platforms.unix;
        };
      };
      nu-alias-converter = craneLib.buildPackage commonArgs;
    in {
      default = nu-alias-converter;
      inherit nu-alias-converter;
    });

    apps = forEachSupportedSystem ({pkgs}: {
      default = {
        type = "app";
        program = "${self.packages.${pkgs.system}.nu-alias-converter}/bin/nuit";
      };
      nuit = {
        type = "app";
        program = "${self.packages.${pkgs.system}.nu-alias-converter}/bin/nuit";
      };
    });

  };
}
