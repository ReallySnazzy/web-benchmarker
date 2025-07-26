{ pkgs, lib, config, inputs, ... }:

{
  languages.rust.enable = true;
  processes.webapp.exec = "cd webapp && cargo run --release";
}
