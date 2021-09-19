use std::collections::HashMap;
use crate::error::*;

#[ derive(Default, Debug) ]
pub struct Props {
    pub map: HashMap<String, String>,
}

impl Props {
    pub fn new(s: &str) -> Result<Self>
    {
        println!("Parse props '{}'", s);
        let mut r = Props { map: HashMap::new() };
        if s == "" { return Ok(r); }
        for i in s.split(";") {
            let eq = i.find('=').ok_or(Error::from("Missing ="))?;
            r.map.insert(i[..eq].to_string(), i[eq + 1..].to_string());
        }
        Ok( r )
    }

    pub fn get(&self, key: &str) -> Option<&str> { return self.map.get(key).map(|s| s as &str) }

    pub fn as_string(&self) -> String
    {
        let mut r = String::new();
        for (key, value) in &self.map {
            if r.len() > 0 { r += ";"; }
            r += &format!("{}={}", key, value)
        }
        return r;
    }
}

#[ derive(Default, Debug) ]
pub struct Url {
    pub proto: String,
    pub host: String,
    pub props: Props,
}

impl Url {
    pub fn new(s: &str) -> Result<Self>
    {
        let psep = s.find("://").ok_or(Error::from("Missing ://"))?;
        let proto = s[..psep].to_string();
        match s[psep..].find(';') {
            Some( sep ) => Ok( Url { proto: proto, host: s[psep + 3..psep + sep].to_string(), props: Props::new(&s[psep + sep + 1..])? } ),
            None => Ok( Url { proto: proto, host: s[psep..].to_string(), props: Props::default() } )
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> { return self.props.get(key) }
}
