{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
   # ...
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.lapack
    pkgs.blas
    # Any other tools you need
  ];

    shellHook = ''
        export RUSTFLAGS="-C link-arg=-v"
    '';
}

