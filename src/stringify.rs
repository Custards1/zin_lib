#[derive(Debug)]
pub enum Error{
    InvalidUnicode(std::num::ParseIntError),
    UnkownUnicode(u32),
    InvalidEscape(String)
}
impl From<std::num::ParseIntError> for Error {
    fn from(a:std::num::ParseIntError)->Self{
        Error::InvalidUnicode(a)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self,f: &mut std::fmt::Formatter<'_>)->std::fmt::Result {
        match self {
            Error::InvalidEscape(a)=>write!(f,"Invalid escape {}",a),
            Error::UnkownUnicode(a)=>write!(f,"Unkown unicode {:X}",a),
            Error::InvalidUnicode(a)=>write!(f,"Invalid unicode number {}",a)
        }
    }
}
type Result<T> = std::result::Result<T,Error>;
macro_rules! some_or_esc_err {
    ($a:expr,$raw:ident) => {
        match $a {
            Some(b)=>b,
            _=>return Err(Error::InvalidEscape($raw.to_string()))
        }       
    };
}
///unescape attempts to escape a non-quoted string
pub fn unescape(string:&str)->Result<String> {
    let mut output = String::with_capacity(string.len());
    let mut raw = string.chars();
    while let Some(c) = raw.next() {
        match c {
            '\\'=>output.push(
                match some_or_esc_err!(raw.next(),string){
                    'a'=>'\u{07}',
                    'n'=>'\n',
                    'b'=>'\u{08}',
                    'v'=>'\u{0b}',
                    'f'=>'\u{0C}',
                    'r'=>'\r',
                    '\n'=>continue,
                    '\t'=>continue,
                    't'=>'\t',
                    'e'=>'\u{1b}',
                    '\\'=>'\\',
                    '\''=>'\'',
                    '"'=>'"',
                    '$'=>'$',
                    '`'=>'`',
                    'u'=>{
                        let lbrack = some_or_esc_err!(raw.next(),string);
                        if lbrack != '{' {
                            return Err(Error::InvalidEscape(string.to_string()))
                        }
                        let mut temp = String::with_capacity(4);
                        temp.push(some_or_esc_err!(raw.next(),string));
                        temp.push(some_or_esc_err!(raw.next(),string));
                        let rbrack = some_or_esc_err!(raw.next(),string);
                        if rbrack != '}' {
                            temp.push(rbrack);
                            temp.push(some_or_esc_err!(raw.next(),string));
                            let frback = some_or_esc_err!(raw.next(),string);
                            if frback != '}'{
                                return Err(Error::InvalidEscape(string.to_string()))
                            }
                        }
                        let num = u32::from_str_radix(&temp, 16)?;
                        let test = std::char::from_u32(num);
                        match test {
                            Some(a)=>a,
                            _=>{
                                return Err(Error::UnkownUnicode(num))
                            }
                        }
                    
                    }
                    _=>return Err(Error::InvalidEscape(string.to_string()))
                }
                ),
            _=>{
                output.push(c);
            }
        };
    }
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_escapes() ->Result<()> {
        assert_eq!("H\neL\"\\o wolrd".to_string(),unescape("H\\neL\\\"\\\\o wolrd")?);
        Ok(())
    }
}