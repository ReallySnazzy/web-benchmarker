{ pkgs, lib, config, inputs, ... }:

{
  languages.python.enable = true;
  languages.python.venv.enable = true;
  languages.python.venv.requirements = ''
    flask
    gunicorn
  '';

  processes.webapp.exec = "gunicorn -w 4 -b 0.0.0.0:5000 app:app";
}
