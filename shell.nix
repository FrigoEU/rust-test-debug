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
  cpptools= pkgs.vscode-extensions.ms-vscode.cpptools;
  codelldb = pkgs.vscode-extensions.vadimcn.vscode-lldb;
  codelldb-wrapped = pkgs.writeShellScriptBin "codelldbwrapped.sh" ''
    ${codelldb}/share/vscode/extensions/vadimcn.vscode-lldb/adapter/codelldb --port=4471
'';
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
    cpptools
    codelldb-wrapped
  ];
  # (setq dap-cpptools-debug-path (getenv "CPP_DAP_DEBUG_BIN_PATH"))
  CPP_DAP_DEBUG_BIN_PATH = cpptools + "/share/vscode/extensions/ms-vscode.cpptools/debugAdapters/bin/OpenDebugAD7";
  LLDB_DAP_DEBUG_BIN_PATH = codelldb-wrapped + "/bin/codelldbwrapped.sh";
}
