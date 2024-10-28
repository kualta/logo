{ pkgs, lib, config, inputs, ... }:

{
  env = {
    GREET = "devenv";
    CARGO_HOME = "${config.env.DEVENV_ROOT}/.cargo";
    RUSTUP_HOME = "${config.env.DEVENV_ROOT}/.rustup";
  };

  packages = with pkgs; [
    git
    gcc
    pkg-config
    openssl
    cargo-cross
    rustup
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "rustfmt" "rust-src" ];
  };

  enterShell = ''
    rustup default stable
    rustup target add x86_64-unknown-linux-gnu
    hello
    git --version
  '';

  scripts = {
    hello.exec = "echo hello from $GREET";
    build-linux.exec = "cross build --target x86_64-unknown-linux-gnu --release";
  };
}
