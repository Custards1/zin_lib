use zin_lib::error::*;
use zin_lib::object::interpreter::Executor;
use zin_lib::types::*;
use std::io::Read;
#[inline]
fn usage() {
    println!(
        concat!("{name} version {version}{newline}{name} cli version {cli_version}{newline}{newline}",
                "   USAGE:{newline}\t{name} [FLAGS..] [COMMMAND | SCRIPT] [PASSED_ARGS...]{newline}",
                "   [OPTIONS]:{newline}",
                "\t-s, --skip_first                          skips first line in file{newline}",
                "\t-v, --version                             prints the {name} version{newline}",
                "\t-V, --cli-version                         prints the {name} cli version{newline}",
                "\t-h, --help                                prints this message{newline}",
                "   [COMMAND]:{newline}",
                "\t-x <command>, --execute <command>         executes the code passed in{newline}",
                "   [SCRIPT]:{newline}",
                "\tthe file name of the script to execute{newline}",
                "   [PASSED_ARGS]:{newline}",
                "\tthe remaining arguments after a [SCRIPT] or [COMMAND] will be passed{newline}\tinto the script{newline}"),
        version = zin_lib::VERSION,
        name = env!("CARGO_PKG_NAME"),
        newline = if cfg!(windows) {"\r\n"} else {"\n"},
        cli_version = env!("CARGO_PKG_VERSION"),
    )
}

enum ScriptType {
    File(std::path::PathBuf),
    Command(String),
    Stdin
}


#[derive(Default)]
struct Options {
    script: Option<ScriptType>,
    script_args:Option<Vec<String>>,
    skip_first:bool,
}
#[inline(always)]
fn isnt_end(idx:usize,len:usize)->bool {
    return idx < len
}
impl Options {
    #[inline]
    fn from_args(mut args: Vec<String>,dont_print:bool) -> Result<Option<Options>> {
        let mut options = Options::default();
        
        let len = args.len();
        for index in 0..len {
            match args[index].as_str() {
                "-x"|"--execute"=> if isnt_end(index+1,len) {
                    let mut opts:Vec<String> = args.drain(index..).collect();
                    let arg:String  = opts.remove(1);
                    options.script = Some(ScriptType::Command(arg));
                    options.script_args = Some(opts);
                    return Ok(Some(options))
                } else {
                    if !dont_print {
                        usage();
                    }
                    return Ok(None)
                }
                "-v"|"--version" => {
                    println!("{version}",version=zin_lib::VERSION);
                }
                "-V"|"--cli-version"=> {
                    println!("{cli_version}",cli_version=env!("CARGO_PKG_VERSION"));
                }
                "-s"|"--skip_first" =>{
                    options.skip_first = true
                }
                "-h"|"--help"=>{
                    if !dont_print {
                        usage();
                    }
                    return Ok(None)
                }
                "--"|"-" =>{
                    options.script = Some(ScriptType::Stdin);
                    let opts:Vec<String> =args.drain(index..).collect();
                    options.script_args = Some(opts);
                    return Ok(Some(options))
                }
                _=>{
                    options.script = Some(ScriptType::File(std::path::PathBuf::from(&args[index])));
                    let opts:Vec<String> =args.drain(index..).collect();
                    options.script_args = Some(opts);
                    return Ok(Some(options))
                }
            }
        }
        Ok(Some(options))
    }
}

#[inline]
fn zmain() ->Result<Value> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = Options::from_args(args, false)?;
    let mut exec:Executor; 
    match options {
        Some(a)=>{
            if let Some(script) = a.script {
                let script = match script {
                    ScriptType::File(path)=>{
                        if !path.exists() {
                            return Err(Error::BadPath(path))
                        } else if path.is_dir() {
                            return Err(Error::ExpectedFileGotDirectory(path))
                        }
                        let path = std::fs::canonicalize(path)?;
                        exec= Executor::new(match path.parent() {
                            Some(p)=>match p.to_str() {
                                Some(a)=>{
                                   
                                    a.into()
                                },
                                _=>return Err(Error::BadPath(path))
                            }
                            _=>return Err(Error::BadPath(path))
                        });
                        exec.script(std::fs::read_to_string(path)?)
                    },
                    ScriptType::Command(code)=>{
                        exec= Executor::new(match std::env::current_dir()?.to_str() {
                            Some(a)=>a.into(),
                            _=>return Err(Error::CustomSingle("NoCurrentDir".into()))
                        });
                        exec.script(code)
                    },
                    ScriptType::Stdin=>{
                        let mut string =String::new();
                        match std::io::stdin().read_to_string(&mut string) {
                            Ok(_)=>{}
                            Err(e)=>return Err(Error::Io(e))
                        }
                        exec= Executor::new(match std::env::current_dir()?.to_str() {
                            Some(a)=>a.into(),
                            _=>return Err(Error::CustomSingle("NoCurrentDir".into()))
                        });
                        exec.script(string)
                    }
                }?;
                if let Some(args) = a.script_args {
                    exec.set_argv(args);
                }
                
                exec.call(script, Vec::with_capacity(0))
            } else {
                Ok(Value::NULL)
            }
        },
        _=>Ok(Value::NULL)
    }
}
fn main() {
    std::process::exit(match zmain() {
        Ok(a)=>match a{
            Value::NULL =>0,
            Value::Bool(b)=>if b{0}else{1},
            Value::Object(_)|Value::String(_)|Value::Bytes(_)|Value::Char(_)=>0,
            Value::Num(n)=>n.i32()
        }
        Err(e)=>{
            eprintln!("{}",e);
            2
        }
    });
}
