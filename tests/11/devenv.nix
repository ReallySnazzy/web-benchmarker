{ pkgs, lib, config, inputs, ... }:

{
  languages.java = {
    enable = true;
    jdk.package = pkgs.jdk21;
    gradle.enable = true;
  };

  processes.webapp.exec = "./gradlew run";
}
