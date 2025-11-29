use "http_server"
use "valbytes"
use "net"

actor Main
  new create(env: Env) =>
    let config = ServerConfig(
      where host' = "0.0.0.0", port' = "3000"
    )
    Server(
      TCPListenAuth(env.root),
      Notify(env),
      SimpleHandlerFactory[MyHandler],
      config
    )

class Notify is ServerNotify
  let _env: Env

  new iso create(env: Env) =>
    _env = env

  fun ref listening(server: Server ref) =>
    _env.out.print("Listening")

  fun ref not_listening(server: Server ref) =>
    _env.out.print("Failed to listen")

  fun ref closed(server: Server ref) =>
    _env.out.print("Server closed")

class MyHandler is HandlerWithoutContext
  let _session: Session

  new create(session: Session) =>
    _session = session

  fun ref apply(request: Request val, request_id: RequestID): Any =>
    let body = "Hello, World!"
    _session.send_raw(
      Responses.builder()
        .set_status(StatusOK)
        .add_header("Content-Type", "text/plain")
        .add_header("Content-Length", body.size().string())
        .finish_headers()
        .add_chunk(body)
        .build(),
      request_id
    )
    _session.send_finished(request_id)
