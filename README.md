## Original code, and how it run

![](img/module10-async.png)

In this experiment I ran one server and three clients all connected to `ws://127.0.0.1:2000`. Whenever any client types a message, the server immediately rebroadcasts it to every connected client. In the screenshot we can see each client printing lines like “127.0.0.1:60229 says: syifa”, “127.0.0.1:60230 says: anada”, and “127.0.0.1:60231 says: matcha” in the order they were sent. The server console also logs each reception, for example “127.0.0.1:60229 Received: syifa”. To reproduce this, open one terminal and run `cargo run --bin server`, then open three more and run `cargo run --bin client` in each, anything we type in one client will instantly appear in the others.
