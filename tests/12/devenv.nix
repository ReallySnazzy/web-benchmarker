{ pkgs, lib, config, inputs, ... }:

{
  languages.ruby.enable = true;

  processes.webapp.exec = "cd webapp && bundle install && bundle exec ruby app.rb";
}
