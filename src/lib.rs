use json_syntax::{FragmentRef, Parse, Value};

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug)]
pub struct JValueMap {
    t: JType,
    region: (usize, usize),
    value: String
}

impl JValueMap {
    pub fn new(_r: (usize, usize), _v: String, _t: JType) -> Self {
        return JValueMap { t: _t, region: _r, value: _v }
    }

    pub fn region(&self) -> (usize, usize) {
        self.region
    }

    pub fn r#type(&self) -> &JType {
        &self.t
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}


pub fn json_values_byte_offsets(json_bytes: Vec<u8>) -> Result<Vec<JValueMap>, String> {
    let json_str = String::from_utf8(json_bytes)
        .map_err(|_| "could not parse bytes as json string".to_owned())?;

    let (value, code_map) = Value::parse_str(&json_str)
        .map_err(|_| "could not parse bytes as json".to_owned())?;

    let mut json_values: Vec<JValueMap> = Vec::new();
    // CodeMap index `i` corresponds to the fragment at traverse offset `i`.
    for (i, frag) in value.traverse() {
        let span = code_map.as_slice()[i].span;
        let (s, e) = (span.start(), span.end());
        let raw = json_str[s..e].to_string();
        match frag {
            FragmentRef::Value(Value::String(_)) => json_values.push(JValueMap::new((s + 1, e - 1), raw, JType::JString)),
            FragmentRef::Value(Value::Number(_)) => json_values.push(JValueMap::new((s, e), raw, JType::JNumber)),
            FragmentRef::Value(Value::Boolean(_)) => json_values.push(JValueMap::new((s, e), raw, JType::JBool)),
            FragmentRef::Value(Value::Null) => json_values.push(JValueMap::new((s, e), raw, JType::JNull)),
            _ => {} // keys, entries, arrays, objects
        }
    }

    Ok(json_values)
}
