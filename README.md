DEBUGGING:

Check shell.nix! Cargo, gdb, etc...

CPP_DAP_DEBUG_BIN_PATH is important, this cause "Doing vfork: ... unknown ..." error. Still need to do (setq dap-cpptools-debug-path (getenv "CPP_DAP_DEBUG_BIN_PATH")) manually for now, should probably add to dir-locals.

Check launch.json for configuration (first one, second is just an example). Program is somehow not respected, so you need to navigate to it manually first time. Then after just do dap-debug-last.

Adding watch expressions -> Right click on window (dont know keybind)
