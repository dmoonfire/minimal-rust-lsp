extern crate gen_lsp_server;
extern crate languageserver_types;
extern crate failure;
extern crate crossbeam_channel;

use crossbeam_channel::{Sender, Receiver};
use languageserver_types::{ServerCapabilities, InitializeParams, request::{GotoDefinition, GotoDefinitionResponse}};
use gen_lsp_server::{stdio_transport, handle_shutdown, RawMessage, RawResponse};

fn main() -> Result<(), failure::Error> {
    let (receiver, sender, io_threads) = stdio_transport();
    gen_lsp_server::run_server(
        ServerCapabilities::default(),
        receiver,
        sender,
        main_loop,
    )?;
    io_threads.join()?;
    Ok(())
}

fn main_loop(
    _params: InitializeParams,
    receiver: &Receiver<RawMessage>,
    sender: &Sender<RawMessage>,
) -> Result<(), failure::Error> {
    for msg in receiver {
        match msg {
            RawMessage::Request(req) => {
                let req = match handle_shutdown(req, sender) {
                    None => return Ok(()),
                    Some(req) => req,
                };
                let req = match req.cast::<GotoDefinition>() {
                    Ok((id, _params)) => {
                        let resp = RawResponse::ok::<GotoDefinition>(
                            id,
                            &Some(GotoDefinitionResponse::Array(Vec::new())),
                        );
                        sender.send(RawMessage::Response(resp));
                        continue;
                    },
                    Err(req) => req,
                };
                // ...
            }
            RawMessage::Response(_resp) => (),
            RawMessage::Notification(_not) => (),
        }
    }
    Ok(())
}

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
