#![feature(core, plugin, test)]
#![plugin(serde_macros)]

extern crate serde;
extern crate "rustc-serialize" as rustc_serialize;
extern crate test;

use std::fmt::Debug;
use std::collections::HashMap;
use test::Bencher;

use rustc_serialize::{Decoder, Decodable};

use serde::de::{Deserializer, Deserialize};

//////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Debug)]
pub enum Error {
    EndOfStream,
    SyntaxError,
    OtherError(String),
}

//////////////////////////////////////////////////////////////////////////////

mod decoder {
    use std::collections::HashMap;
    use std::collections::hash_map::IntoIter;
    use rustc_serialize;

    use super::Error;
    use super::Error::{EndOfStream, SyntaxError, OtherError};
    use self::Value::{StringValue, IsizeValue};

    enum Value {
        StringValue(String),
        IsizeValue(isize),
    }

    pub struct IsizeDecoder {
        len: usize,
        iter: IntoIter<String, isize>,
        stack: Vec<Value>,
    }

    impl IsizeDecoder {
        #[inline]
        pub fn new(values: HashMap<String, isize>) -> IsizeDecoder {
            IsizeDecoder {
                len: values.len(),
                iter: values.into_iter(),
                stack: vec!(),
            }
        }
    }

    impl rustc_serialize::Decoder for IsizeDecoder {
        type Error = Error;

        fn error(&mut self, msg: &str) -> Error {
            OtherError(msg.to_string())
        }

        // Primitive types:
        fn read_nil(&mut self) -> Result<(), Error> { Err(SyntaxError) }
        fn read_usize(&mut self) -> Result<usize, Error> { Err(SyntaxError) }
        fn read_u64(&mut self) -> Result<u64, Error> { Err(SyntaxError) }
        fn read_u32(&mut self) -> Result<u32, Error> { Err(SyntaxError) }
        fn read_u16(&mut self) -> Result<u16, Error> { Err(SyntaxError) }
        fn read_u8(&mut self) -> Result<u8, Error> { Err(SyntaxError) }
        #[inline]
        fn read_isize(&mut self) -> Result<isize, Error> {
            match self.stack.pop() {
                Some(IsizeValue(x)) => Ok(x),
                Some(_) => Err(SyntaxError),
                None => Err(EndOfStream),
            }
        }
        fn read_i64(&mut self) -> Result<i64, Error> { Err(SyntaxError) }
        fn read_i32(&mut self) -> Result<i32, Error> { Err(SyntaxError) }
        fn read_i16(&mut self) -> Result<i16, Error> { Err(SyntaxError) }
        fn read_i8(&mut self) -> Result<i8, Error> { Err(SyntaxError) }
        fn read_bool(&mut self) -> Result<bool, Error> { Err(SyntaxError) }
        fn read_f64(&mut self) -> Result<f64, Error> { Err(SyntaxError) }
        fn read_f32(&mut self) -> Result<f32, Error> { Err(SyntaxError) }
        fn read_char(&mut self) -> Result<char, Error> { Err(SyntaxError) }
        #[inline]
        fn read_str(&mut self) -> Result<String, Error> {
            match self.stack.pop() {
                Some(StringValue(x)) => Ok(x),
                Some(_) => Err(SyntaxError),
                None => Err(EndOfStream),
            }
        }

        // Compound types:
        fn read_enum<T, F>(&mut self, _name: &str, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_enum_variant<T, F>(&mut self, _names: &[&str], _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder, usize) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_enum_variant_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_enum_struct_variant<T, F>(&mut self, _names: &[&str], _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder, usize) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_enum_struct_variant_field<T, F>(&mut self, _f_name: &str, _f_idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_struct<T, F>(&mut self, _s_name: &str, _len: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_struct_field<T, F>(&mut self, _f_name: &str, _f_idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_tuple<T, F>(&mut self, _len: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_tuple_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_tuple_struct<T, F>(&mut self, _s_name: &str, _len: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        fn read_tuple_struct_arg<T, F>(&mut self, _a_idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(Error::SyntaxError)
        }

        // Specialized types:
        fn read_option<T, F>(&mut self, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder, bool) -> Result<T, Error>,
        {
            Err(SyntaxError)
        }

        fn read_seq<T, F>(&mut self, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder, usize) -> Result<T, Error>,
        {
            Err(SyntaxError)
        }

        fn read_seq_elt<T, F>(&mut self, _idx: usize, _f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            Err(SyntaxError)
        }

        #[inline]
        fn read_map<T, F>(&mut self, f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder, usize) -> Result<T, Error>,
        {
            let len = self.len;
            f(self, len)
        }
        #[inline]
        fn read_map_elt_key<T, F>(&mut self, _idx: usize, f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            match self.iter.next() {
                Some((key, value)) => {
                    self.stack.push(IsizeValue(value));
                    self.stack.push(StringValue(key));
                    f(self)
                }
                None => {
                    Err(SyntaxError)
                }
            }
        }

        #[inline]
        fn read_map_elt_val<T, F>(&mut self, _idx: usize, f: F) -> Result<T, Error> where
            F: FnOnce(&mut IsizeDecoder) -> Result<T, Error>,
        {
            f(self)
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

mod deserializer {
    use std::collections::HashMap;
    use std::collections::hash_map::IntoIter;

    use super::Error;
    use super::Error::{EndOfStream, SyntaxError};
    use self::State::{StartState, KeyOrEndState, ValueState, EndState};

    use serde::de;

    #[derive(PartialEq, Debug)]
    enum State {
        StartState,
        KeyOrEndState,
        ValueState(isize),
        EndState,
    }

    pub struct IsizeDeserializer {
        stack: Vec<State>,
        len: usize,
        iter: IntoIter<String, isize>,
    }

    impl IsizeDeserializer {
        #[inline]
        pub fn new(values: HashMap<String, isize>) -> IsizeDeserializer {
            IsizeDeserializer {
                stack: vec!(StartState),
                len: values.len(),
                iter: values.into_iter(),
            }
        }
    }

    impl Iterator for IsizeDeserializer {
        type Item = Result<de::Token, Error>;

        #[inline]
        fn next(&mut self) -> Option<Result<de::Token, Error>> {
            match self.stack.pop() {
                Some(StartState) => {
                    self.stack.push(KeyOrEndState);
                    Some(Ok(de::Token::MapStart(self.len)))
                }
                Some(KeyOrEndState) => {
                    match self.iter.next() {
                        Some((key, value)) => {
                            self.stack.push(ValueState(value));
                            Some(Ok(de::Token::String(key)))
                        }
                        None => {
                            self.stack.push(EndState);
                            Some(Ok(de::Token::End))
                        }
                    }
                }
                Some(ValueState(x)) => {
                    self.stack.push(KeyOrEndState);
                    Some(Ok(de::Token::Isize(x)))
                }
                Some(EndState) => {
                    None
                }
                None => {
                    None
                }
            }
        }
    }

    impl de::Deserializer<Error> for IsizeDeserializer {
        #[inline]
        fn end_of_stream_error(&mut self) -> Error {
            EndOfStream
        }

        #[inline]
        fn syntax_error(&mut self, _token: de::Token, _expected: &[de::TokenKind]) -> Error {
            SyntaxError
        }

        #[inline]
        fn unexpected_name_error(&mut self, _token: de::Token) -> Error {
            SyntaxError
        }

        #[inline]
        fn conversion_error(&mut self, _token: de::Token) -> Error {
            SyntaxError
        }

        #[inline]
        fn missing_field<
            T: de::Deserialize<IsizeDeserializer, Error>
        >(&mut self, _field: &'static str) -> Result<T, Error> {
            Err(SyntaxError)
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

fn run_decoder<
    D: Decoder<Error=Error>,
    T: Clone + PartialEq + Debug + Decodable
>(mut d: D, value: T) {
    let v = Decodable::decode(&mut d);

    assert_eq!(Ok(value), v);
}

#[bench]
fn bench_decoder_000(b: &mut Bencher) {
    b.iter(|| {
        let m: HashMap<String, isize> = HashMap::new();
        run_decoder(decoder::IsizeDecoder::new(m.clone()), m)
    })
}

#[bench]
fn bench_decoder_003(b: &mut Bencher) {
    b.iter(|| {
        let mut m: HashMap<String, isize> = HashMap::new();
        for i in range(0is, 3) {
            m.insert(i.to_string(), i);
        }
        run_decoder(decoder::IsizeDecoder::new(m.clone()), m)
    })
}

#[bench]
fn bench_decoder_100(b: &mut Bencher) {
    b.iter(|| {
        let mut m: HashMap<String, isize> = HashMap::new();
        for i in range(0is, 100) {
            m.insert(i.to_string(), i);
        }
        run_decoder(decoder::IsizeDecoder::new(m.clone()), m)
    })
}

fn run_deserializer<
    D: Deserializer<E>,
    E: Debug,
    T: Clone + PartialEq + Debug + Deserialize<D, E>
>(mut d: D, value: T) {
    let v: T = Deserialize::deserialize(&mut d).unwrap();

    assert_eq!(value, v);
}

#[bench]
fn bench_deserializer_000(b: &mut Bencher) {
    b.iter(|| {
        let m: HashMap<String, isize> = HashMap::new();
        run_deserializer(deserializer::IsizeDeserializer::new(m.clone()), m)
    })
}

#[bench]
fn bench_deserializer_003(b: &mut Bencher) {
    b.iter(|| {
        let mut m: HashMap<String, isize> = HashMap::new();
        for i in range(0is, 3) {
            m.insert(i.to_string(), i);
        }
        run_deserializer(deserializer::IsizeDeserializer::new(m.clone()), m)
    })
}

#[bench]
fn bench_deserializer_100(b: &mut Bencher) {
    b.iter(|| {
        let mut m: HashMap<String, isize> = HashMap::new();
        for i in range(0is, 100) {
            m.insert(i.to_string(), i);
        }
        run_deserializer(deserializer::IsizeDeserializer::new(m.clone()), m)
    })
}
