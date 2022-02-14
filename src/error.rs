use super::zstring::*;
#[derive(Debug)]
pub enum Error {
    ParserError(String),
    ParserErrorNear(String,std::ops::Range<usize>,String,usize),
    Io(std::io::Error),
    InvalidPtr(usize),
    InvalidFunctionPtr(usize),
    InvalidBounds(usize),
    ExpectedFuncPtr(usize),
    BadStackFrame,
    InvalidJump,
    MissingReturn,
    MalformedStack,
    ExpectedFileGotDirectory(std::path::PathBuf),
    BadPath(std::path::PathBuf),
    BadFunctionValue(String),
    UnknownVariable(String),
    InvalidBorrow(&'static str,u32),
    Unconvertable(ZString,ZString),
    FunctionDne(String),
    InvalidCode,
    InvalidSelf(String),
    InvalidArgs(usize,usize),
    InvalidCall(String),
    AssertionFailed(String),
    NoImportSystem,
    Utf8Error(std::str::Utf8Error),
    Custom(String,String),
    CustomSingle(String),
    InvalidUnwind,
    InvalidWind,
    InvalidErrorType
}




impl Default for Error {
    fn default()->Self {
        Self::ParserError("unknown error".to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(a:std::io::Error)->Error {
        Error::Io(a)
    }
}
impl From<std::str::Utf8Error> for Error {
    fn from(a:std::str::Utf8Error)->Error {
        Error::Utf8Error(a)
    }
}
pub type Result<I> = std::result::Result<I,Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"zin error: ")?;
        match self {
            Error::ParserError(a)=>write!(f," syntax error: {}",a),
            Error::ParserErrorNear(a,b,c,d)=>write!(f," syntax error: {} near {} on line {} in {}..{}",a,c,d,b.start,b.end),
            Error::Io(a)=>write!(f," io error: {}",a),
            Error::InvalidPtr(a)=>write!(f,"ivp (remove this error with useful error)"),
            Error::InvalidFunctionPtr(a)=>write!(f,"(remove this error with useful error)"),
            Error::InvalidBounds(a)=>write!(f," access out of bounds at: {}",a),
            Error::ExpectedFuncPtr(a)=>write!(f,"efp (remove this error with useful error)"),
            Error::BadStackFrame=>write!(f,"bsf (remove this error with useful error)"),
            Error::InvalidJump=>write!(f,"ivj (remove this error with useful error)"),
            Error::MissingReturn=>write!(f,"mr (remove this error with useful error)"),
            Error::MalformedStack=>write!(f,"malformed stack"),
            Error::ExpectedFileGotDirectory(a)=>write!(f,"expected file, got directory: {:?}",a),
            Error::BadPath(a)=>write!(f,"path {:?} does not exist",a),
            Error::BadFunctionValue(a)=>write!(f,"bad function value: {}",a),
            Error::UnknownVariable(a)=>write!(f,"unkown variable: {}",a),
            Error::InvalidBorrow(a,b)=>write!(f,"(remove this error with useful error) invalid borrow in {} on line {}",a,b),
            Error::Unconvertable(a,b)=>write!(f,"(remove this error with useful error) unconvertable in {} on line {}",a,b),
            Error::InvalidCode=>write!(f,"(remove this error with useful error) invalid code"),
            Error::InvalidSelf(a)=>write!(f,"no value was provided for 'self', got {}",a),
            Error::InvalidArgs(a,b)=>write!(f,"invalid args: function expects {} args but {} were given",b,a),
            Error::FunctionDne(a)=>write!(f,"function does not exist {}",a),
            Error::InvalidCall(a)=>write!(f,"attempted to call non function {}",a),
            Error::AssertionFailed(a)=>write!(f,"assertion failed: {}",a),
            Error::NoImportSystem=>write!(f,"no import system!"),
            Error::Utf8Error(a)=>write!(f,"UTF-8 error: {}",a),
            Error::Custom(a,b)=>write!(f,"{}: {}",a,b),
            Error::CustomSingle(a)=>write!(f,"{}",a),
            Error::InvalidUnwind=>write!(f,"invalid stack unwind on no error"),
            Error::InvalidWind=>write!(f,"invalid stack wind on no error"),
            Error::InvalidErrorType=>write!(f,"invalid error type")
        }
    } 
}
impl Error{
    #[inline]
    pub fn kind(&self)-> &'static str {
        match self { 
            Error::CustomSingle(_) | Error::Custom(_,_)=>"CustomError",
            Error::ParserError(_)|Error::ParserErrorNear(_,_,_,_)=>"ParserError",
            Error::Io(_)=>"IoError",
            Error::InvalidPtr(_)|Error::InvalidFunctionPtr(_)=>"PointerError",
            Error::InvalidBounds(_)=>"InvalidBounds",
            Error::ExpectedFuncPtr(_)|Error::InvalidCall(_)|Error::BadFunctionValue(_)|Error::FunctionDne(_)=>"InvalidCall",
            Error::BadStackFrame|Error::MalformedStack=>"CallStackError",
            Error::InvalidJump=>"JumpError",
            Error::MissingReturn=>"ReturnError",
            Error::ExpectedFileGotDirectory(_)|Error::BadPath(_)=>"PathError",
            Error::UnknownVariable(_)=>"VarUnknownError",
            Error::InvalidBorrow(_,_)|Error::Unconvertable(_,_)|Error::InvalidCode=>"TypeError",
            Error::InvalidSelf(_)=>"SelfError",
            Error::InvalidArgs(_,_)=>"ArgsError",
            Error::AssertionFailed(_)=>"AssertionError",
            Error::NoImportSystem=>"ImportError",
            Error::Utf8Error(_)=>"Utf8Error",
            Error::InvalidUnwind=>"UnwindError",
            Error::InvalidWind=>"WindError",
            Error::InvalidErrorType=>"ErrorError",
        }
    }
    #[inline]
    pub fn arg(&self) ->String {
        format!("{}",self)
    }
}
#[macro_export]
macro_rules! impossible {
    () => {
        panic!("Internal zin impossibisity")
    };
}