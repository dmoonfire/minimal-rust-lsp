## Previous Notes

- I copied the dependencies from the [gen_lsp_server's Cargo.toml](https://github.com/rust-analyzer/rust-analyzer/blob/master/crates/gen_lsp_server/Cargo.toml) and added the [gen_lsp_server](https://crates.io/crates/gen_lsp_server) to the dependencies.
- I switched the `lsp-types` to `languageserver-types 0.51.1` because it failed the build and I saw the line in the `cargo build` command.
- I removed the `extern crate` lines.
- I removed the `run_server` from the `use` line since we use `gen_lsp_server::run_server` directly.

## Previous Problems

When I try to build the example, I get this:

```
/*
error[E0631]: type mismatch in function arguments
  --> src\main.rs:12:5
   |
12 |       gen_lsp_server::run_server(
   |       ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected signature of `fn(languageserver_types::InitializeParams, &crossbeam_channel::internal::channel::Receiver<gen_lsp_server::RawMessage>, &crossbeam_channel::internal::channel::Sender<gen_lsp_serv
er::RawMessage>) -> _`
...
22 | / fn main_loop(
23 | |     _params: InitializeParams,
24 | |     receiver: &Receiver<RawMessage>,
25 | |     sender: &Sender<RawMessage>,
...  |
51 | |     Ok(())
52 | | }
   | |_- found signature of `for<'r, 's> fn(languageserver_types::InitializeParams, &'r crossbeam_channel::Receiver<gen_lsp_server::RawMessage>, &'s crossbeam_channel::Sender<gen_lsp_server::RawMessage>) -> _`
   |
   = note: required by `gen_lsp_server::run_server`

error[E0308]: mismatched types
  --> src\main.rs:30:54
   |
30 |                 let req = match handle_shutdown(req, sender) {
   |                                                      ^^^^^^ expected struct `crossbeam_channel::internal::channel::Sender`, found struct `crossbeam_channel::Sender`
   |
   = note: expected type `&crossbeam_channel::internal::channel::Sender<gen_lsp_server::RawMessage>`
              found type `&crossbeam_channel::Sender<gen_lsp_server::RawMessage>`

error: aborting due to 2 previous errors
    */
```

It appears to be an issue of `crossbeam_channel::internal:channel` references instead of `crossbeam_channel`.
