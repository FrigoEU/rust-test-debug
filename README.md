DEBUGGING:

Check shell.nix! Cargo, gdb, etc...

CPP_DAP_DEBUG_BIN_PATH is important, this cause "Doing vfork: ... unknown ..." error. We're setting this in dir-locals, env var "CPP_DAP_DEBUG_BIN_PATH" comes from shell.nix

Check launch.json for configuration. Program is somehow not respected, so you need to navigate to it manually first time. Then after just do dap-debug-last.

Adding watch expressions -> Right click on window (dont know keybind) , or better yet, use dap-hydra;

Watch out when opening the project: first open a non-rust file, then a rust file, otherwise order of loading (lorri, direnv, emacs, dir-locals, etc) gets screwed up

Tried furiously to get debugging with lldb working, but I just can't manage. I feel like everything seems to be working, dap-mode just isn't starting up correctly? Or the wrapping in shell.nix doesn't work? I also don't need it anymore, since I just wanted it to pretty print vectors etc, and I got it to work with gdb using: "gdbpath" in launch.json. Yes!
