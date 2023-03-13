mkdir ~/.config/nushell
ln -sf (readlink -e .devcontainer/nushell/env.nu) ~/.config/nushell/env.nu
ln -sf (readlink -e .devcontainer/nushell/config.nu) ~/.config/nushell/config.nu
