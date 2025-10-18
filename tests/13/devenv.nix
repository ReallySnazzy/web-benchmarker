{ pkgs, lib, config, inputs, ... }:

{
  languages.rust.enable = true;
  processes.webserver.exec = "cd web && cargo run --release";
}
