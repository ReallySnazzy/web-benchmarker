let handler _socket request _body =
  match Http.Request.resource request with
  | "/" -> Cohttp_eio.Server.respond_string ~status:`OK ~body:"Hello, world!" ()
  | _ -> Cohttp_eio.Server.respond_string ~status:`Not_found ~body:"" ()

let log_warning ex = Logs.warn (fun f -> f "%a" Eio.Exn.pp ex)

let () =
  let port = ref 8080 in
  Arg.parse
    [ ("-p", Arg.Set_int port, " Listening port number(8080 by default)") ]
    ignore "An HTTP/1.1 server";
  Eio_main.run @@ fun env ->
  Eio.Switch.run @@ fun sw ->
  Printf.printf "Listening on 0.0.0.0:%d\n" !port;
  flush_all();
  let socket =
    Eio.Net.listen env#net ~sw ~backlog:128 ~reuse_addr:true
      (`Tcp (Eio.Net.Ipaddr.V4.any, !port))
  and server = Cohttp_eio.Server.make ~callback:handler () in
  Cohttp_eio.Server.run socket server ~on_error:log_warning
