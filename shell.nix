{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = with pkgs; [
        dbus
        glib
        cairo
        atk
        pango
        gdk-pixbuf
        gtk3
        alsa-lib
        udev
    ];
    nativeBuildInputs = with pkgs.buildPackages; [ 
	pkg-config
	dbus
    ];
    LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
      "/run/opengl-driver"
      "/run/opengl-driver-32"
      pkgs.libGL
      pkgs.vulkan-loader
    ];
}
