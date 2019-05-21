use crossbeam_channel::{Receiver, Sender};
use failure::*;
use gen_lsp_server::{handle_shutdown, run_server, stdio_transport, RawMessage, RawResponse};
use log::*;
use lsp_types::{
    request::{GotoDefinition, GotoDefinitionResponse},
    InitializeParams, ServerCapabilities,
};
use std::io::BufRead;

mod msg;
mod stdio;

fn main() -> Result<(), failure::Error> {
    simple_logger::init()?;
    info!("starting example server");

    // A limited version.
    {
        let (receiver, _sender, io_threads) = crate::stdio::stdio_transport();

        trace!("before internal receiver.recv");
        let req = match receiver.recv() {
            Ok(req) => req,
            msg => bail!("expected internal initialize request, got {:?}", msg),
        };
        info!("req {:?}", req);
        io_threads.join()?;
        info!("after join");
    }

    // This seems to work and shows the proper debug message and fails because we asked it to.
    /*
    {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();
        info!("stdin");
        let buffer = read_msg_text(&mut stdin)?;
        info!("buffer {:?}", buffer);
    }*/

    // This is pulled out from the gen_lsp_serv::server. This just hangs and never gives even
    // the debug message that it got something.
    /*
    {
        let (receiver, _sender, io_threads) = stdio_transport();

        trace!("before receiver.recv");
        let req = match receiver.recv() {
            Ok(req) => req,
            msg => bail!("expected initialize request, got {:?}", msg),
        };
        info!("req {:?}", req);
        io_threads.join()?;
        info!("after join");
    }
    */

    // Original version I'm trying to get work.
    /*
    let (receiver, sender, io_threads) = stdio_transport();
    gen_lsp_server::run_server(ServerCapabilities::default(), receiver, sender, main_loop)?;
    io_threads.join()?;
    */

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
                info!("got request {:?}", req);
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
                    }
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
