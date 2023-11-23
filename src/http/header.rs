use crate::http::query_string::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Header<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> Header<'buf> {
    pub fn get(&self, key: &'buf str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for Header<'buf> {
    fn from(request: &'buf str) -> Self {
        let mut data = HashMap::new();
        let mut raw_header = request;
        loop {
            if let Some(i) = raw_header.find("\r\n") {
                let line = &raw_header[..i];
                raw_header = &raw_header[i + 2..];

                if let Some(j) = line.find(":") {
                    let key = line[..j].trim();
                    let val = line[j + 2..].trim();

                    data.entry(key)
                        .and_modify(|existing: &mut Value| match existing {
                            Value::Single(prev_val) => {
                                *existing = Value::Multiple(vec![prev_val, val]);
                            }
                            Value::Multiple(vec) => vec.push(val),
                        })
                        .or_insert(Value::Single(val));
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Self { data }
    }
}

pub struct HeaderError;
