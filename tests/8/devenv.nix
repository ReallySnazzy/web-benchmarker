{ pkgs, lib, config, inputs, ... }:

{
  languages.javascript = {
    enable = true;
    bun.enable = true;
  };
  processes.webapp.exec = "cd webapp && bun run main.ts";
}
