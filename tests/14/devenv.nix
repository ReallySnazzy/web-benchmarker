{ pkgs, lib, config, inputs, ... }:

{
  packages = [ pkgs.ponyc pkgs.pony-corral ];
  processes.webserver.exec = "cd web && corral fetch && corral run -- ponyc -Dopenssl_3.0.x && ./web";
}
