defmodule Webapp.Endpoint do
  use Phoenix.Endpoint, otp_app: :webapp

  plug Webapp.Router
end
