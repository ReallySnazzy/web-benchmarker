defmodule Webapp.Router do
  use Phoenix.Router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", Webapp do
    get "/", PageController, :index
  end
end
