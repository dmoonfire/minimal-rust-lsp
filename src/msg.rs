use failure::format_err;
use std::io::BufRead;

pub fn read_msg_text(
    inp: &mut impl BufRead,
) -> std::result::Result<Option<String>, failure::Error> {
    let mut size = None;
    let mut buf = String::new();
    loop {
        buf.clear();
        if inp.read_line(&mut buf)? == 0 {
            return Ok(None);
        }
        if !buf.ends_with("\r\n") {
            panic!("malformed header: {:?}", buf);
        }
        let buf = &buf[..buf.len() - 2];
        if buf.is_empty() {
            break;
        }
        let mut parts = buf.splitn(2, ": ");
        let header_name = parts.next().unwrap();
        let header_value = parts
            .next()
            .ok_or_else(|| format_err!("malformed header: {:?}", buf))?;
        if header_name == "Content-Length" {
            size = Some(header_value.parse::<usize>()?);
        }
    }
    let size = size.ok_or_else(|| format_err!("no Content-Length"))?;
    let mut buf = buf.into_bytes();
    buf.resize(size, 0);
    inp.read_exact(&mut buf)?;
    let buf = String::from_utf8(buf)?;
    log::debug!("< {}", buf);
    Ok(Some(buf))
}
