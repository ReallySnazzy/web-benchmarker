{ pkgs, lib, config, inputs, ... }:

{
  languages.ocaml.enable = true;
  packages = with pkgs; [ 
    openssl
    gmp
  ];
  processes.webserver.exec = "cd web && dune exec bin/main.exe";
}
