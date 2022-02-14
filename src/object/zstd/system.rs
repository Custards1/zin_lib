
use crate::types::*;
use crate::object::*;
use crate::error::*;
use function::Function;
use interpreter::Executor;
use super::builtin;
pub mod property {

    static_tstring!(os);
    static_tstring!(arch);
    static_tstring!(family);
    static_tstring!(exe_suffix);
    static_tstring!(exe_ext);
    static_tstring!(lib_suffix);
    static_tstring!(lib_prefix);
    static_tstring!(lib_ext);
    static_tstring!(zin_ext);
    static_tstring!(zin_alt_ext);
    static_tstring!(extensions);   
    static_tstring!(env);
    static_tstring!(import_loaders);
    static_tstring!(home);
    static_tstring!(pictures);
    static_tstring!(templates);
    static_tstring!(videos);
    static_tstring!(audio);
    static_tstring!(runtime);
    static_tstring!(cache);
    static_tstring!(data);
    static_tstring!(data_local);
    static_tstring!(desktop);
    static_tstring!(documents);
    static_tstring!(downloads);
    static_tstring!(executables);
    static_tstring!(fonts);
    static_tstring!(prefrences);
    static_tstring!(public);
    static_tstring!(config);
    static_tstring!(zin_path);
    static_tstring!(path);
    static_tstring!(zin_imports);
    static_tstring!(reflect);
    static_tstring!(get_attr);
    static_tstring!(set_attr);
    static_tstring!(dis);
    static_tstring!(pointer);
}

const SYS_NAME: &'static str = "system";
const SYS_DOC:&'static str = 
"The system module contains information about the hosts platform, and the zin runtime system";
pub mod env {
    use super::*;
    const ENV_NAME:&'static str = "env";

    const ENV_DOC:&'static str = 
    "The env module contains the host platforms environment variables, and provides methods to access them";

   
    fn get_env(exec:Executor,args:Vec<Value>)->Result<Value> {
        match args.len() {
            0=>Ok(Value::NULL),
            _=>{
                let a:&str =&args[0].invoke_to_string(&exec);
                if a.is_empty()|| a.contains('=') || a.contains('\0') {
                    Ok(Value::NULL)
                } else {
                    Ok(match std::env::var(a){
                        Ok(value) => Value::Object(builtin::string_from(value)),
                        _=>Value::NULL
                    })
                }
            }
        }
    }    
    fn set_env(exec:Executor,args:Vec<Value>)->Result<Value> {
        match args.len() {
            0|1=>Ok(Value::Bool(false)),
            _=>{
                let a:&str =&args[0].invoke_to_string(&exec);
                let b:&str =&args[1].invoke_to_string(&exec);
                if (a.is_empty()|| a.contains('=') || a.contains('\0')) || b.contains('=') || b.contains('\0'){
                    Ok(Value::Bool(false))
                } else {
                    std::env::set_var(a,b);
                    Ok(Value::Bool(true))
                }
            }
        }
    }
    #[inline]
    fn env()->GcObject{
        GcObject::from(Object::new(access::ObjAccess::NONE)).with_doc(ENV_DOC)
        .with_member(builtin::property::accessors::__get(), Function::native_root(access::ObjAccess::NONE,get_env))
        .with_member(builtin::property::accessors::__set(), Function::native_root(access::ObjAccess::NONE,set_env))
    }
    #[inline]
    pub fn open_env(obj:GcObject)->GcObject{
        obj.with_member(ENV_NAME, env())
    }
}
pub mod platform {
    use super::*;
    const PLATFORM_NAME:&'static str = "platform";
    const PLATFORM_DOCS:&'static str = "Contains information about the hosts platform. Includes information such as os,arch,platform...";
     
    pub mod extensions {
        use super::*;
        const ZIN_EXT_DEFAULT :&'static str = "zin";
        const ZIN_EXT :&'static str = "ZIN_EXT";
        const ZIN_ALT_EXT :&'static str = "ZIN_ALT_EXT";
        const ZIN_ALT_EXT_DEFAULT :&'static str = "z";
        
        const EXTENSIONS_NAME :&'static str= "extensions"; 
        const EXTENSIONS_DOCS :&'static str= "extensions contains information about the file extentions used for both shared libraries and executables"; 
        #[inline]
        pub fn zin_ext()->ZString {
            match std::env::var(ZIN_EXT){
                Ok(a)=> if a.is_empty() {
                    ZString::from(ZIN_EXT_DEFAULT)
                } else {
                    ZString::from(a.as_str())
                }
                _=>ZString::from(ZIN_EXT_DEFAULT)
            }
        }
        #[inline]
        pub fn zin_alt_ext()->ZString {
            match std::env::var(ZIN_ALT_EXT){
                Ok(a)=> if a.is_empty() {
                    ZString::from(ZIN_ALT_EXT_DEFAULT)
                } else {
                    ZString::from(a.as_str())
                }
                _=>ZString::from(ZIN_ALT_EXT_DEFAULT)
            }
        }
        #[inline]
        pub fn lib_ext()->ZString {
            ZString::from(std::env::consts::DLL_EXTENSION)
        }
        #[inline]
        pub fn lib_prefix()->ZString {
            ZString::from(std::env::consts::DLL_PREFIX)
        }
        #[inline]
        pub fn lib_suffix()->ZString {
            ZString::from(std::env::consts::DLL_SUFFIX)
        }
        #[inline]
        pub fn exe_ext()->ZString {
            ZString::from(std::env::consts::EXE_EXTENSION)
        }
        #[inline]
        pub fn exe_suffix()->ZString {
            ZString::from(std::env::consts::EXE_SUFFIX)
        }
        #[inline]
        fn ext()->GcObject {
            GcObject::from(Object::new(access::ObjAccess::NONE)).with_doc(EXTENSIONS_DOCS)
            .with_member(property::exe_suffix(),exe_suffix())
            .with_member(property::exe_ext(),exe_ext())
            .with_member(property::lib_prefix(),lib_prefix())
            .with_member(property::lib_suffix(),lib_suffix())
            .with_member(property::lib_ext(),lib_ext())
            .with_member(property::zin_ext(),zin_ext())
            .with_member(property::zin_alt_ext(),zin_alt_ext())
            
        }
        #[inline]
        pub fn open_ext(obj:GcObject)->GcObject {
            obj.with_member(EXTENSIONS_NAME,ext())
        }
    }
    
    
    #[inline]
    pub fn arch()->ZString {
        ZString::from(std::env::consts::ARCH)
    }
    #[inline]
    pub fn family()->ZString {
        ZString::from(std::env::consts::FAMILY)
    }
    #[inline]
    pub fn os()->ZString {
        ZString::from(std::env::consts::OS)
    }
    #[inline]
    fn platform()->GcObject {
        extensions::open_ext(GcObject::from(Object::new(access::ObjAccess::NONE)).with_doc(PLATFORM_DOCS)
        .with_member(property::os(),os())
        .with_member(property::arch(),arch())
        .with_member(property::family(),family()))
    }
    pub fn open_platform(obj: GcObject) -> GcObject {
        obj.with_member(PLATFORM_NAME,platform())
    }
}




pub mod library {
    use super::*;
    const LIBRARY_NAME: &'static str = "library";
    const LIBRARY_DOC: &'static str = "library contains commonly used paths, such as users cache,desktop,pictures,home...";
    
    #[inline]
    pub fn home()->Value {
        match dirs::home_dir()       {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
              }
    #[inline]
    pub fn audio()->Value {
        match dirs::audio_dir()      {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
              }
    #[inline]
    pub fn pictures()->Value {
        match dirs::picture_dir()    {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn cache()->Value {
        match dirs::cache_dir()      {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
              }
    #[inline]
    pub fn config()->Value  {
        match dirs::config_dir()     {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn data()->Value {
        match dirs::data_dir()       {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn data_local()->Value {
        match dirs::data_local_dir() {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn desktop()->Value {
        match dirs::desktop_dir()    {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn documents()->Value{
        match dirs::document_dir()   {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn downloads()->Value {
        match dirs::download_dir()   {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn executables()->Value {
        match dirs::executable_dir()    {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn fonts()->Value {
        match dirs::font_dir()       {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn prefrences()->Value {
        match dirs::preference_dir() {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn public()->Value {
        match dirs::public_dir()     {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn runtime()->Value {
        match dirs::runtime_dir()    {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn template()->Value {
        match dirs::template_dir()   {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    pub fn videos()->Value {
        match dirs::video_dir()      {
            Some(val)=>match val.to_str() {
                Some(a)=>a.into(),
                _=>Value::NULL

            },
            _=>Value::NULL
        }
    }
    #[inline]
    fn library()->GcObject {
        GcObject::from(Object::new(access::ObjAccess::NONE)).with_doc(LIBRARY_DOC)
        .with_member(property::videos(),videos())
        .with_member(property::templates(),template())
        .with_member(property::runtime(),runtime())
        .with_member(property::public(),public())
        .with_member(property::prefrences(),prefrences())
        .with_member(property::fonts(),fonts())
        .with_member(property::executables(),executables())
        .with_member(property::downloads(),downloads())
        .with_member(property::documents(),documents())
        .with_member(property::desktop(),desktop())
        .with_member(property::data(),data())
        .with_member(property::data_local(),data_local())
        .with_member(property::config(),config())
        .with_member(property::cache(),cache())
        .with_member(property::pictures(),pictures())
        .with_member(property::audio(),audio())
        .with_member(property::home(),home())
        
    }
    #[inline]
    pub fn open_library(obj: GcObject)->GcObject {
        obj.with_member(LIBRARY_NAME,library())
    }
    
}
pub mod runtime {
    use super::*;
    const ZIN_HOME: &'static str = "ZIN_HOME";
    const RUNTIME_DOC: &'static str = "contains the zin system runtime";
    
    #[inline]
    pub fn zin_path()->Value {
        match std::env::var(ZIN_HOME) {
            Ok(a)=>Value::String(a.into()),
            _=>match dirs::home_dir()      {
                Some(mut val)=>{
                    val.push(".zin");
                    match val.to_str() {
                        Some(a)=>a.into(),
                        _=>Value::NULL
                    }
                },
                _=>Value::NULL
            }
        }
    }
   

    #[inline]
    fn inspect_fields(_:Executor,args:Vec<Value>)->Result<Value> {
        Ok(match args.len() {
            0=>Value::NULL,
            _=>match &args[0] {
                Value::Object(a)=> match a.get_fields() {
                    Some(a)=>builtin::vec_from(a).into(),
                    _=>Value::NULL
                }
                _=>Value::NULL
            }
        })
    }
    #[inline]
    fn get_field(_:Executor,args:Vec<Value>)->Result<Value> {
        Ok(match args.len() {
            0|1=>Value::NULL,
            _=>match &args[0] {
                Value::Object(a)=> match a.member_unchecked(args[1].clone()) {
                    Some(a)=>a.clone(),
                    _=>Value::NULL
                }
                _=>Value::NULL
            }
        })
    }
    #[inline]
    fn pointer(_:Executor,args:Vec<Value>)->Result<Value> {
        Ok(match args.len() {
            0=>Value::NULL,
            _=>match &args[0] {
                Value::Object(a)=> Value::String(ZString::from(format!("{:p}",a.as_ref()))),
                _=>Value::NULL
            }
        })
    }
    #[inline]
    fn set_field(_:Executor,args:Vec<Value>)->Result<Value> {
        Ok(match args.len() {
            0|1|2=>Value::Bool(false),
            _=>{
                let (mut mself,mut args) = {
                    let mut hashed_iter_ggvcc = args.iter();
                    match hashed_iter_ggvcc.next() {
                        Some(a)=>match a {
                            crate::types::Value::Object(b)=>Ok((b.clone(),hashed_iter_ggvcc)),
                            _=>Err(crate::error::Error::InvalidSelf(format!("{} ",a)))
                        },
                        _=>Err(crate::error::Error::InvalidSelf(format!("None")))
                    }
                }?;
                let key = args.next().unwrap().clone();
                let val = args.next().unwrap().clone();
                mself.set_member_value(key,val);
                Value::Bool(true)
            }
        })
    }
    
    #[inline]
    fn runtime(zp:Value)->GcObject{
        
        let sys = GcObject::from(Object::new(access::ObjAccess::NONE)).with_doc(RUNTIME_DOC);
        sys.with_member(property::zin_path(),zp.clone())
        
       
        .with_member(property::reflect(),Function::native_root(access::ObjAccess::NONE, inspect_fields) )
        .with_member(property::get_attr(), Function::native_root(access::ObjAccess::NONE, get_field))
        .with_member(property::set_attr(), Function::native_root(access::ObjAccess::NONE, set_field))
        .with_member(property::pointer(), Function::native_root(access::ObjAccess::NONE, pointer))
        .with_member(property::dis(), Function::native_root(access::ObjAccess::NONE, builtin::glob_implication::_inspect_code))
    }
    #[inline]
    pub fn open_runtime(obj:GcObject,zp:Value) -> GcObject{
        obj.with_member(property::runtime(), runtime(zp))
    }
}

#[inline]
fn populate_partial_std(builtins:&mut PropertyMap,json:Value,io:Value,ast:Value,zin:Value,path:Value) {
    builtins.insert(Value::String("json".into()),json);
    builtins.insert(Value::String("io".into()),io);
    builtins.insert(Value::String("ast".into()),ast);
    builtins.insert(Value::String("zin".into()),zin);
    builtins.insert(Value::String("path".into()),path);
}
#[inline]
fn populate_remaining_std(builtins:&mut PropertyMap,system:Value,builtin:Value) {
    builtins.insert(Value::String("system".into()),system);
    builtins.insert(Value::String("builtin".into()),builtin);
}
#[inline]
fn remaining_std(partial:&mut PropertyMap,builtins:&mut PropertyMap,system:Value,builtin:Value) {
    populate_remaining_std(partial,system.clone(),builtin.clone());
    populate_remaining_std(builtins,system,builtin);
}
#[inline]
fn partial_std(builtins:&mut PropertyMap)->PropertyMap {
    let mut a = PropertyMap::new();
    let json:Value = super::json::json().into();
    let io:Value = super::io::io().into();
    let ast:Value = super::ast::ast().into();
    let zin:Value = super::zin::open_zin().into();
    let path:Value = super::zpath::open_path().into();
    populate_partial_std(&mut a,json.clone(),io.clone(),ast.clone(),zin.clone(),path.clone());
    populate_partial_std(builtins,json.clone(),io.clone(),ast.clone(),zin.clone(),path.clone());
    a
}

#[inline]
pub fn zin_import_path(script_dir:&ZString,zin_path:&Value)->GcObject {
    let mut map =  vec![script_dir.clone().into()];
    if !zin_path.is_null() {
        map.push(zin_path.clone())
    }
    builtin::vec_from(map)
}
#[inline]
pub fn system(builtin:GcObject,script_dir:&ZString)->GcObject {
    let mut sys = GcObject::from(Object::new(access::ObjAccess::NONE).with_name(SYS_NAME).with_doc(SYS_DOC))
    .with_member("import", Function::native_root(access::ObjAccess::NONE,super::import_system::import_wrapper))
    .with_member("package_spec", Function::native_root(access::ObjAccess::NONE,super::import_system::locate_wrapper));
    let mut builtins = PropertyMap::new();
    let mut partial = partial_std(&mut builtins);
    remaining_std(&mut partial,&mut builtins,sys.clone().into(),builtin.into());
    sys.set_member("preload", builtin::map_from(builtins));
    sys = env::open_env(sys) .with_member(property::zin_imports(),builtin::map_from(partial));
    sys = platform::open_platform(sys);
    sys = library::open_library(sys);
    let zp = runtime::zin_path();
    sys = runtime::open_runtime(sys,zp.clone());
    sys.with_member(property::path(),zin_import_path(&script_dir,&zp))
}
#[inline]
pub fn open_system(builtin:GcObject,script_dir:&ZString)->GcObject {
    system(builtin,script_dir)
}