use crossbeam_channel::{bounded, Receiver, Sender};
use failure::*;
use log::*;
use std::{
    io::{stdin, stdout, BufRead, Write},
    thread,
};

mod msg;

fn main() -> Result<(), failure::Error> {
    simple_logger::init()?;
    info!("starting example server");

    // A limited version.
    {
        let (receiver, _sender, io_threads) = stdio_transport();

        trace!("before internal receiver.recv");
        let req = match receiver.recv() {
            Ok(req) => req,
            msg => bail!("expected internal initialize request, got {:?}", msg),
        };
        info!("req {:?}", req);
        io_threads.join()?;
        info!("after join");
    }

    Ok(())
}

pub fn stdio_transport() -> (Receiver<String>, Sender<String>, Threads) {
    trace!("stdio_transport");

    let (writer_sender, writer_receiver) = bounded::<String>(16);
    let writer = thread::spawn(move || {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        writer_receiver.into_iter().for_each(|it| {
            info!("Got server output: {:?}", it);
            stdout.write(it.as_ref()).unwrap();
        });
        Ok(())
    });
    let (reader_sender, reader_receiver) = bounded::<String>(16);
    let reader = thread::spawn(move || {
        let stdin = stdin();
        let mut stdin = stdin.lock();
        while let Some(msg) = read(&mut stdin)? {
            info!("Got user input: {:?}", msg);
            reader_sender.send(msg).unwrap();
        }
        Ok(())
    });
    let threads = Threads { reader, writer };
    (reader_receiver, writer_sender, threads)
}

pub struct Threads {
    reader: thread::JoinHandle<Result<(), failure::Error>>,
    writer: thread::JoinHandle<Result<(), failure::Error>>,
}

impl Threads {
    pub fn join(self) -> Result<(), failure::Error> {
        match self.reader.join() {
            Ok(r) => r?,
            Err(_) => bail!("reader panicked"),
        }
        match self.writer.join() {
            Ok(r) => r,
            Err(_) => bail!("writer panicked"),
        }
    }
}

pub fn read(r: &mut impl BufRead) -> Result<Option<String>, failure::Error> {
    let mut buf = String::new();

    if r.read_line(&mut buf)? == 0 {
        return Ok(None);
    }

    Ok(Some(buf))
}
