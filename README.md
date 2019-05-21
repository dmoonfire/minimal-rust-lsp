# Minimal LSP Server in Rust

I'm trying to get a minimal LSP server working using [gen_lsp_server](https://crates.io/crates/gen_lsp_server).

## Manual Testing



### Initialize the request.

```
Content-Length: 87

{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "capabilities": {} }}
```

### Goto Definition

```
Content-Length: 66

{"jsonrpc": "2.0", "id": 1, "method": "textDocument/definition"}
```
