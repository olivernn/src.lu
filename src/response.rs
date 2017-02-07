extern crate rocket;

use rocket::response::Response;
use rocket::http::ContentType;
use rocket::http::hyper::header::{CacheControl, CacheDirective};

use std::vec::Vec;
use std::io;
use std::io::{Write, Read, Seek};
use std::cmp;

pub struct Image {
    pub raw: Vec<u8>,
    pos: u64
}

impl Image {
    pub fn new() -> Image {
        Image{ raw: vec!(), pos: 0}
    }
}

impl Write for Image {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let start = self.raw.len();
        self.raw.extend_from_slice(buf);
        let end = self.raw.len();
        Ok(end - start)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for Image {
    fn read(&mut self, mut buf: &mut[u8]) -> io::Result<usize> {
        let start = self.pos as usize;
        let end = cmp::min(start + 4096, self.raw.len() as usize) as usize;
        let chunk = &self.raw[start..end];

        self.pos = end as u64;

        buf.write(chunk)
    }
}

impl Seek for Image {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match pos {
            io::SeekFrom::Start(n) => self.pos = n,
            io::SeekFrom::End(n) => self.pos = (self.raw.len() as i64 + n) as u64,
            io::SeekFrom::Current(n) => self.pos = (self.pos as i64 + n) as u64,
        };

        Ok(self.pos)
    }
}

impl<'r> rocket::response::Responder<'r> for Image {
    fn respond(self) -> Result<Response<'r>, rocket::http::Status> {
        Response::build()
            .header(ContentType::PNG)
            .header(CacheControl(vec![CacheDirective::MaxAge(86400)]))
            .sized_body(self)
            .ok()
    }
}

