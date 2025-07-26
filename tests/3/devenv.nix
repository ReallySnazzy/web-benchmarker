{ pkgs, lib, config, inputs, ... }:

{
  languages.dotnet.enable = true;
  processes.webapp.exec = "cd webapp && dotnet run";
}
