{ pkgs, lib, config, inputs, ... }:

{
  languages.elixir.enable = true;

  processes.webapp.exec = "cd webapp && mix deps.get && mix phx.server";

  packages = [
    pkgs.beam27Packages.hex
  ];
}
