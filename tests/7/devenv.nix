{ pkgs, lib, config, inputs, ... }:

{
  languages.ocaml.enable = true;
  processes.webapp.exec = "cd webapp && dune exec bin/main.exe";
}
