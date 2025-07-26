{ pkgs, lib, config, inputs, ... }:

{
  languages.rust.enable = true;

  scripts.run-bench.exec = ''
    cd test-runner
    cargo build --release
    cd ..
    ./test-runner/target/release/test-runner
  '';

  packages = with pkgs; [
    rewrk
    openssl
  ];
}
