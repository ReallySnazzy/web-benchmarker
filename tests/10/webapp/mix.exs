defmodule Webapp.MixProject do
  use Mix.Project

  def project do
    [
      app: :webapp,
      version: "0.1.0",
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      mod: {Webapp.Application, []},
      extra_applications: [:logger, :runtime_tools]
    ]
  end

  defp deps do
    [
      {:phoenix, "~> 1.7.10"},
      {:plug_cowboy, "~> 2.7"}
    ]
  end
end
