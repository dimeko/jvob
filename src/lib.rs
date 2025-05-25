use std::{io::{BufReader, Read, Seek}};
use json_syntax::{CodeMap, Parse, Value};
use std::io::Cursor;

#[derive(Debug, PartialEq, Clone)]
pub enum JType {
    JString,
    JNumber,
    JBool,
    JNull
}

pub trait TrimString {
    fn trim_edges_bytes(&self) -> Option<String>;
}

impl TrimString for String {
    fn trim_edges_bytes(&self) -> Option<String> {
        let bytes = self.as_bytes();
        if bytes.len() <= 2 {
            return None;
        }
        std::str::from_utf8(&bytes[1..bytes.len() - 1])
            .ok()
            .map(|s| s.to_string())
    }
}

pub struct JValueMap {
    t: JType,
    region: (usize, usize),
    value: String
}

impl JValueMap {
    fn new(_r: (usize, usize), _v: String, _t: JType) -> Self {
        return JValueMap { t: _t, region: _r, value: _v }
    }

    pub fn region(self) -> (usize, usize) {
        self.region
    }

    pub fn r#type(&self) -> JType {
        self.t.clone()
    }

    pub fn value(self) -> String {
        self.value
    }
}

pub fn json_values_byte_offsets(json_bytes: Vec<u8>) -> Result<Vec<JValueMap>, String> {
    let cursor = Cursor::new(json_bytes);
    let mut reader = BufReader::new(cursor);

    let mut _json_bytes_vec = Vec::new();
    let skip_code_map = ['{', '}', '[', ']', ':'];
    let mut json_values : Vec<JValueMap> = Vec::new();
    reader.read_to_end(&mut _json_bytes_vec).unwrap();

    let _: Result<(Value, CodeMap), json_syntax::parse::Error> = match Value::parse_str(String::from_utf8(_json_bytes_vec.clone()).unwrap().as_str()) {
        Ok(_p) => {
            'outer: for _map in _p.1 {
                reader.seek(std::io::SeekFrom::Start(0)).unwrap();

                let mut _rev: i64 = (_map.1.span.end() - _map.1.span.start() ) as i64;
                let mut tmp_value = String::new();
                for (pos, mut _jb) in reader.by_ref().bytes().enumerate().skip(_map.1.span.start()).take(_map.1.span.end() - _map.1.span.start() ) {
                    if _json_bytes_vec.len() <= ((pos as i64) + _rev) as usize { _rev = _rev - 2; continue;}
                    if skip_code_map.contains(&(_json_bytes_vec[((pos as i64) + _rev) as usize] as char)) {
                        _rev = _rev - 2;
                        continue 'outer;
                    }
                    tmp_value.push(_json_bytes_vec[pos] as char);
                    _rev = _rev - 2;
                }
                let _type: JType;
                match tmp_value.chars().nth(0).unwrap() {
                    '\"' => {
                        json_values.push(JValueMap::new((_map.1.span.start(), _map.1.span.end()), tmp_value, JType::JString));
                    },
                    't' | 'f' => {
                        json_values.push(JValueMap::new((_map.1.span.start(), _map.1.span.end()), tmp_value, JType::JBool));
                    },
                    'n' => {
                        json_values.push(JValueMap::new((_map.1.span.start(), _map.1.span.end()), tmp_value, JType::JNull));
                    },
                    c => {
                        if c.is_ascii_digit() {
                            json_values.push(JValueMap::new((_map.1.span.start(), _map.1.span.end()), tmp_value, JType::JNumber));
                        }
                    }                    
                }
            }

            return Ok(json_values);
        },
        Err(_r) => {
            return Err("could not parse btes as json".to_owned());
        }
    };
}