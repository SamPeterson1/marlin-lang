{ pkgs ? import <nixpkgs> {} }:

let
  llvm = pkgs.llvmPackages_18;
in
pkgs.mkShell {
  packages = with pkgs; [
    llvm.llvm.dev     # provides llvm-config
    llvm.lld

    libffi
    zlib
    ncurses
    libxml2
    pkg-config
  ];

  # REQUIRED: tells llvm-sys exactly which llvm-config to use
  LLVM_SYS_181_PREFIX = llvm.llvm.dev;
  LLVM_CONFIG_PATH = "${llvm.llvm.dev}/bin/llvm-config";
}
