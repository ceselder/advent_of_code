pkgs.mkShell {
   # ...

   RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}
