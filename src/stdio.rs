use crossbeam_channel::{bounded, Receiver, Sender};
use failure::*;
use log::*;
use serde_json::from_str;
use std::{
    io::{self, stdin, stdout, BufRead, Write},
    thread,
};

pub fn stdio_transport() -> (Receiver<String>, Sender<String>, Threads) {
    trace!("stdio_transport");

    let (writer_sender, writer_receiver) = bounded::<String>(16);
    let writer = thread::spawn(move || {
        // If I uncomment this, it locks up.
        let stdout = stdout();
        let mut stdout = stdout.lock();
        writer_receiver.into_iter().for_each(|it| {
            stdout.write(it.as_ref()).unwrap();
        });
        Ok(())
    });
    let (reader_sender, reader_receiver) = bounded::<String>(16);
    let reader = thread::spawn(move || {
        let stdin = stdin();
        let mut stdin = stdin.lock();
        while let Some(msg) = read(&mut stdin)? {
            error!("input {:?}", msg);
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
    let text = match crate::msg::read_msg_text(r)? {
        None => return Ok(None),
        Some(text) => text,
    };
    Ok(Some(text))
}
