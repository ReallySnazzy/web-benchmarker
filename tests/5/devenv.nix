{ pkgs, lib, config, inputs, ... }:

{
  languages.deno.enable = true;
  processes.webapp.exec = 'cd webapp && deno run -A main.ts'
}
