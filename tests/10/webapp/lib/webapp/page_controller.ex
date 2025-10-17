defmodule Webapp.PageController do
  use Phoenix.Controller

  def index(conn, _params) do
    text(conn, "Hello, World!")
  end
end
