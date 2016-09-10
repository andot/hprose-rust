/**********************************************************\
|                                                          |
|                          hprose                          |
|                                                          |
| Official WebSite: http://www.hprose.com/                 |
|                   http://www.hprose.org/                 |
|                                                          |
\**********************************************************/
/**********************************************************\
 *                                                        *
 * io/writer.rs                                           *
 *                                                        *
 * hprose writer for Rust.                                *
 *                                                        *
 * LastModified: Sep 11, 2016                             *
 * Author: Chen Fei <cf@hprose.com>                       *
 *                                                        *
\**********************************************************/

extern crate test;

use std::i32;
use std::io::Write;
use std::string::String;

use super::tags::*;
use super::encoder::*;

pub struct Writer {
    buf: Vec<u8>
}

impl Writer {
    pub fn serialize<T: Encoder>(&mut self, v: T) -> &mut Writer {
        self.write_value(v);
        self
    }

    pub fn write_value<T: Encoder>(&mut self, v: T) {
        v.encode(self);
    }

    pub fn write_bool(&mut self, b: bool) {
        self.buf.push(if b { TagTrue } else { TagFalse });
    }

    pub fn write_int(&mut self, i: i64) {
        if i >= 0 && i <= 9 {
            self.buf.push('0' as u8 + i as u8);
            return
        }
        if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
            self.buf.push(TagInteger);
        } else {
            self.buf.push(TagLong);
        }
        write!(self.buf, "{}", i);
        self.buf.push(TagSemicolon);
    }

    pub fn string(&mut self) -> String {
        String::from_utf8(self.buf.clone()).unwrap()
    }

    #[inline]
    pub fn new() -> Writer {
        Writer {
            buf: Vec::with_capacity(1024)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::super::tags::*;
    use super::*;
    use super::test::Bencher;

    #[test]
    fn test_serialize_true() {
        let mut writer = Writer::new();
        writer.serialize(true);
        assert_eq!(writer.string(), "t");
    }

    #[bench]
    fn benchmark_serialize_true(b: &mut Bencher) {
        let mut writer = Writer::new();
        b.iter(|| {
            writer.serialize(true);
        });
    }

    #[bench]
    fn benchmark_write_true(b: &mut Bencher) {
        let mut writer = Writer::new();
        b.iter(|| writer.write_bool(true));
    }

    #[test]
    fn test_serialize_false() {
        let mut writer = Writer::new();
        writer.serialize(false);
        assert_eq!(writer.string(), "f");
    }

    #[test]
    fn test_serialize_int() {
        let mut writer = Writer::new();
        writer.serialize(8);
        assert_eq!(writer.string(), "8");
    }

    #[bench]
    fn benchmark_serialize_int(b: &mut Bencher) {
        let mut writer = Writer::new();
        b.iter(|| {
            writer.serialize(8888);
        });
    }

    #[bench]
    fn benchmark_write_int(b: &mut Bencher) {
        let mut writer = Writer::new();
        b.iter(|| writer.write_int(8888));
    }
}
