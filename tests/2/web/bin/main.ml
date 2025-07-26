let () =
  Dream.run ~interface:"0.0.0.0" ~port:8080
  @@ fun _ -> Dream.html "Hello, world!"
