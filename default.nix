{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage {
  pname = "super-simple-wayland-bar";
  version = "0.1.0";

  src = ./.;

  cargoHash = "sha256-b9eKLoAruyxFPUmL6imej5h+APXXE/7IjXFUvp/FuRM=";

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    gtk4
    glib
    cairo
    pango
    gdk-pixbuf
    gtk4-layer-shell
  ];
}
