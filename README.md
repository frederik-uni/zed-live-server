# zed-live-server

This extension adds my live server to zed

- https://github.com/frederik-uni/live-server-lsp

### Info
- This is a very simple implementation of a live server
- its always running
- i dont know how to use commands(opening the browser) in zed/ maybe not possible yet
- the codeaction shows on which port the live server is running
  
### How it works
- server: file change/save => sends update info over websocket
- css updates style tag, everything else reloads the page
- planed: extension that reloads only when the src was actually loaded fom the server(its only possible to monitor all outgoing requests with a extension ;( )
