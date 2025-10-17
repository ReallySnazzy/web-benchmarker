import Config

config :webapp, Webapp.Endpoint,
  http: [ip: {0, 0, 0, 0}, port: 4000],
  server: true,
  secret_key_base: "a_very_long_secret_key_base_that_should_be_at_least_64_bytes_long_for_security",
  debug_errors: false,
  code_reloader: false

config :logger, level: :warning

config :phoenix, :json_library, Jason
