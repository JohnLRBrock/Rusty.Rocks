### Installation
Using Nixos add `cargo`, `trunk`, and `gcc`, `llc` to your Nixos pkgs and rebuild

### Commands
build: `cargo build`
run: `trunk serve --open`

### Debugging
#### compiled by an incompatible version of rustc
You can see this error when you have installed rust in more than one way. In this case I had installed both cargo and rustup using nixos. Once I uninstalled rustup it built properly
