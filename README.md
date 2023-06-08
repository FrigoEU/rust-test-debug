DEBUGGING:

Check shell.nix! Cargo, gdb, etc...

Check launch.json for configuration. Program is somehow not respected, so you need to navigate to it manually first time. Then after just do dap-debug-last.

Adding watch expressions -> Right click on window (dont know keybind) , or better yet, use dap-hydra;

Using: https://github.com/WebFreak001/code-debug
I think dap-mode installed this automatically somehow?

"warning: Source file is more recent than executable." -> first do cargo build!
