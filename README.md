# zed-live-server

This extension adds my live server to zed

- https://github.com/frederik-uni/live-server-lsp

### Info
- This is a very simple implementation of a live server
- its always running
- i dont know how to use commands(opening the browser) in zed/ maybe not possible yet
- the codeaction opens in browser & shows which port it is running on
  
### How it works
- server: file change/save => sends update info over websocket
- css updates style tag, everything else reloads the page
- planed: extension that reloads only when the src was actually loaded fom the server(its only possible to monitor all outgoing requests with a extension ;( )

### Config

```rs
{
    /// Set if update on save or keypress [Default: false]
    lazy: bool,
    /// 0.0.0.0 or 127.0.0.1 [Default: false]
    public: bool,
    start_port: number
}
```
