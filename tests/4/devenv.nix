{ pkgs, lib, config, inputs, ... }:

{
  languages.go.enable = true;

  processes.webapp.exec = 'go run main.go';
}
