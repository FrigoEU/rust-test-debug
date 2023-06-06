let
  pinnedNixpkgs = import (builtins.fetchTarball {
    name = "nixpkgs-22.11";
    url = https://github.com/NixOS/nixpkgs/archive/22.11.tar.gz;
    # Hash obtained using `nix-prefetch-url --unpack <url>`
    # sha256 = "0mhqhq21y5vrr1f30qd2bvydv4bbbslvyzclhw0kdxmkgg3z4c92";
  }) {};
in
{ pkgs ? pinnedNixpkgs }:
let
  cpp = pkgs.vscode-extensions.ms-vscode.cpptools;
in
pkgs.stdenv.mkDerivation rec {
  name = "rust-projects-test";
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
    pkgs.rust-analyzer
    pkgs.clippy
    pkgs.gdb
    pkgs.nodejs

    # Debugging rust!
    cpp
  ];
  # (setq dap-cpptools-debug-path (getenv "CPP_DAP_DEBUG_BIN_PATH"))
  CPP_DAP_DEBUG_BIN_PATH = cpp + "/share/vscode/extensions/ms-vscode.cpptools/debugAdapters/bin/OpenDebugAD7";
}
