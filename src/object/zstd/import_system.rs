
use crate::types::*;
use crate::object::*;
use crate::error::*;
use function::Function;
use interpreter::Executor;
use std::convert::TryInto;
use super::builtin;
macro_rules! wrap_zimport {
    ($name:ident,$wrapper:expr) => {
        #[inline]
        pub fn $name(e:Executor,args:Vec<Value>)-> Result<Value> {
            match args.len() {
                0=>Err(Error::InvalidArgs(1,0)),
                _=>{
                    let name:ZString = match args[0].clone().try_into(){
                        Ok(a)=>a,
                        _=>return Err(Error::Unconvertable(file!(),line!()))
                    };
                    $wrapper(name)
                }
            }
        }
    }
}

#[inline]
pub fn load_module(e:Executor,args:Vec<Value>)-> Result<Value> {
    match args.len() {
        0=>Err(Error::InvalidArgs(1,0)),
        _=>{
            let module:GcObject = args[0].clone().try_into()?;
            match module.member("filename") {
                Some(a)=>{
                    let s = a.invoke_to_string(&e);
                    let mut func = Function::from(Object::new(access::ObjAccess::NONE)
                    .with_member("__file", s.clone())
                    .with_name(match std::path::Path::new(s.as_ref()).file_name(){
                        Some(a)=>{
                            a.to_str().unwrap_or("__unknown__")
                        }
                        _=>{
                            "__unknown__"
                        }
                    } ));
                    func.consume_script(&e.builtin, std::fs::read_to_string(s.as_ref())?)?;
                    e.call(func.clone().object(), Vec::with_capacity(0))?;
                    e.system_imports_append(s.clone(), func.clone().object());
                    Ok(func.into())
                }
                _=>Err(Error::Custom("ImportError".to_string(),"no filename to open".to_string()))
            }
        }
    }
}
#[inline]
pub fn load_package(e:Executor,args:Vec<Value>)-> Result<Value> {
    match args.len() {
        0=>Err(Error::InvalidArgs(1,0)),
        _=>{
            let mut module:GcObject = args[0].clone().try_into()?;
            let mmod = module.clone();
            let pkgname:Value = module.member("pkgname").unwrap_or("__unknown".into());
            let mut rmod = GcObject::from(Object::new(access::ObjAccess::NONE)
            .with_member("__name",pkgname ));
            let fmod = rmod.clone();
            let mut remaining = Vec::new();
            loop {
                let name:ZString = module.member("pkgname").unwrap_or("__unknown".into()).try_into()?;
                e.system_imports_append(name, rmod.clone());

                match module.member("base") {
                    Some(a)=>{
                        let mut spec:GcObject = match module.member("spec") {
                            Some(a)=>a.try_into()?,
                            _=>return Err(Error::Custom("ImportError".to_string(),"Corrupted package loader".to_string()))
                        };
                        let s = a.invoke_to_string(&e);
                        let mut base = std::path::PathBuf::from(s.as_ref());
                        let init = match base.file_name(){
                            Some(a)=>{
                                a.to_str().unwrap_or("__unknown")
                            }
                            _=>{
                                "__unknown"
                            }
                        };
                        let main =spec.index("main")?; 
                        match &main {
                            Value::String(a)=>{
                                    base.push(a.as_ref());
                                    if base.is_dir() {
                                        let mut xname = match std::path::Path::new(s.as_ref()).file_name(){
                                            Some(a)=>{
                                                a.to_str().unwrap_or("__unknown__")
                                            }
                                            _=>{
                                                "__unknown__"
                                            }
                                        }.to_string();
                                        xname.push('.');
                                        xname.push_str(match base.file_name() {
                                            Some(a)=>{
                                                a.to_str().unwrap_or("__unknown__")
                                            }
                                            _=>{
                                                "__unknown__"
                                            }
                                        });
                                        let xname:ZString = xname.into();
                                        base.push("package.json");
                                        if base.exists() {
                                            
                                            let spe:GcObject = super::json::decode_value(std::fs::read_to_string(&base)?.into())?.try_into()?;
                                            base.pop();
                                            remaining.push((rmod.clone(),package_loader(xname.clone(),base.to_str().unwrap_or("__unknown").into(), spe),
                                            GcObject::from(Object::new(access::ObjAccess::NONE)
                                            .with_member("__name",xname ))));
                                            
                                            base.pop();
                                            continue;
                                        }
                                        base.pop();
                                    }
                                    base.push(".zin");
                                    if !base.exists() {
                                        return Err(Error::Custom("ImportError".to_string(),"Invalid import".to_string()))
                                    }
                                    let bname =base.to_str().unwrap_or("__unknown");
                                    let mut loader = module_loader(bname.into());
                                    let mut xname = s.to_string();
                                    xname.push('.');
                                    xname.push_str(&bname[..bname.len()-4]);
                                    let xname:ZString = xname.into();
                                    loader.set_name(xname.clone());
                                    rmod.set_member(xname,e.call_method(loader, "load".into(),Vec::with_capacity(0))?);
                            }
                            _=>{
                                
                            }
                        };
                        match &spec.index("libs")? {
                            Value::Object(o)=>{
                                let len = o.vec_len()?;
                                
                                for i in 0..len {
                                    base.push(o.index(i)?.invoke_to_string(&e).as_ref());
                                    
                                    if base.is_dir() {
                                        let mut xname = match std::path::Path::new(s.as_ref()).file_name(){
                                            Some(a)=>{
                                                a.to_str().unwrap_or("__unknown__")
                                            }
                                            _=>{
                                                "__unknown__"
                                            }
                                        }.to_string();
                                        xname.push('.');
                                        xname.push_str(match base.file_name() {
                                            Some(a)=>{
                                                a.to_str().unwrap_or("__unknown__")
                                            }
                                            _=>{
                                                "__unknown__"
                                            }
                                        });
                                        let xname:ZString = xname.into();
                                        base.push("package.json");
                                        if base.exists() {
                                            
                                            let spe:GcObject = super::json::decode_value(std::fs::read_to_string(&base)?.into())?.try_into()?;
                                            base.pop();
                                            remaining.push((rmod.clone(),package_loader(xname.clone(),base.to_str().unwrap_or("__unknown").into(), spe),
                                            GcObject::from(Object::new(access::ObjAccess::NONE)
                                            .with_member("__name",xname ))));
                                            
                                            base.pop();
                                            continue;
                                        }
                                        base.pop();
                                    }
                                    base.push(".zin");
                                    if !base.exists() {
                                        return Err(Error::Custom("ImportError".to_string(),"Invalid import".to_string()))
                                    }
                                    let bname =base.to_str().unwrap_or("__unknown");
                                    let mut loader = module_loader(bname.into());
                                    let mut xname = s.to_string();
                                    xname.push('.');
                                    xname.push_str(&bname[..bname.len()-4]);
                                    let xname:ZString = xname.into();
                                    loader.set_name(xname.clone());
                                    rmod.set_member(xname,e.call_method(loader, "load".into(),Vec::with_capacity(0))?);
                                }
                            }
                            _=>{
    
                            }
                        }
    
                        match remaining.pop() {
                            Some((mut r,a,b))=>{
                                r.set_member(b.name().unwrap_or("__unknown".into()), b.clone());
                                rmod = b;
                                module = a;
                            }
                            _=>{
                                return Ok(fmod.into());
                            }
                        }
                        
                        
                    }
                    _=>return Err(Error::Custom("ImportError".to_string(),"no filename to open".to_string()))
                }    
            }
          
        }
    }
}
fn noop(e:Executor,args:Vec<Value>)-> Result<Value> {
    Ok(Value::NULL)
}
#[inline]
pub fn module_loader(filename:ZString) ->GcObject {
    GcObject::from(Object::new(access::ObjAccess::NONE).with_name("ModuleLoader").with_doc("loads a module"))
    .with_member("filename", filename)
    .with_method("load".into(), Function::native_root_method(load_module))
}
#[inline]
pub fn bultin_loader(name:ZString) ->GcObject {
    GcObject::from(Object::new(access::ObjAccess::NONE).with_name("BuiltinLoader").with_doc("dummy loader, does nothing"))
    .with_member("name", name)
    .with_method("load".into(), Function::native_root_method(noop))
}
#[inline]
pub fn package_loader(name:ZString,base:ZString,spec:GcObject) ->GcObject {
    GcObject::from(Object::new(access::ObjAccess::NONE).with_name("PackageLoader").with_doc("loads a package"))
    .with_member("base", base)
    .with_member("spec",spec)
    .with_member("pkgname",name)
    .with_member("load", Function::native_root_method( load_package) )
}



#[inline]
pub fn import_path(path: &std::path::Path) ->Result<Option<Value>> {
    if path.is_dir() {
        let name:ZString = match path.file_name().and_then(|x|x.to_str()) {
            Some(a)=>a,
           _=>return  Err(Error::BadPath(path.into()))
        }.into();
        let base:ZString = match path.to_str() {
            Some(a)=>a,
           _=>return  Err(Error::BadPath(path.into()))
        }.into();
        let mut path = std::path::PathBuf::from(path);
        path.push("package.json");
        match std::fs::read_to_string(&path) {
            Ok(file)=>{
                let spec:GcObject = super::json::decode_value(file.into())?.try_into()?;
                Ok(Some(package_loader(name,base,spec).into()))
            }
            _=>Ok(None)
        }
    } else{
        match path.extension() {
            Some(a)=>{
                if a == std::ffi::OsStr::new("zin") && path.exists() {
                    match path.to_str() {
                        Some(a) =>{
                            return Ok(Some(module_loader(a.into()).into()));
                        }
                        None=>return Err(Error::Custom("ImportError".to_string(),format!("path {:?} does not exist",&path)))
                    }
                    
                }
            }
            _=>{

            }
        };
        Ok(None)
    }
}
#[inline]
pub fn import_protocol(e:&Executor,path:&std::path::Path,name:ZString) ->Result<Option<Value>> {
    let mut buf = std::path::PathBuf::from(path);
    let mut split =name.split('.').map(|x| x.into()).collect::<Vec<ZString>>();
    
    let base = split.drain(0..1).collect::<Vec<ZString>>();
    buf.push(base[0].as_ref());
    match import_path(&buf)? {
        Some(path) =>{
            let obj:Result<GcObject> = path.try_into();
            match obj {
                Ok(a)=>match access_members(a,&split) {
                    Some(a)=>return Ok(Some(a)),
                    _=>return Ok(None)
                },
                _=>return Ok(None)
            }
        },
        _=>{}
    }
    buf.set_extension("zin");
    match import_path(&buf)? {
        Some(path) =>{
            let obj:Result<GcObject> = path.try_into();
            match obj {
                Ok(a)=>match access_members(a,&split) {
                    Some(a)=>return Ok(Some(a)),
                    _=>return Ok(None)
                },
                _=>return Ok(None)
            }
        },
        _=>Ok(None)
    } 
}
#[inline]
pub fn access_members(mods:GcObject,split:&[ZString]) ->Option<Value> {
    let mut mods:Value = mods.into();
    let len = split.len();
    for i in 0..len {
        let mut name:ZString = split[i].clone();
        let obj: GcObject= match mods.try_into() {
            Ok(a)=>a,
            _=>return None
        };
        match obj.member(name) {
            Some(val)=>{
                mods=val;
            }
            _=>return None
        }
    }   
    return Some(mods); 
}
#[inline]
pub fn import_cached(e:&Executor,name:ZString) ->Option<Value> {
    let split =name.split('.').collect::<Vec<_>>();
    if let Some(mods) = e.system_imports_get(split[0].into()) {
        let mut mods:Value = mods.into();
        
        let len = split.len();
        for i in 1..len {
            let mut name:ZString = split[i].into();
            let obj: GcObject= match mods.try_into() {
                Ok(a)=>a,
                _=>return None
            };
            match obj.member(name) {
                Some(val)=>{
                    mods=val;
                }
                _=>return None
            }
        }   
        return Some(mods); 
    }
    None
}
#[inline]
pub fn import_preload(e:&Executor,name:ZString)->Option<Value> {
    Some(e.system_imports_get(name)?.into())
}
#[inline]
pub fn import_preload_builtins_spec(e:&Executor,name:ZString)->Option<Value> {
    Some(e.system_builtin_spec(name)?.into())
}

#[inline]
pub fn import(e:&Executor,name:ZString)->Result<Value> {
    if let Some(value) = import_preload(e, name.clone()) {
        return Ok(value)
    }
    if let Some(value) = import_cached(e, name.clone()) {
        return Ok(value)
    }
    let obj: GcObject= locate_extension(e, name.clone())?.try_into()?;
    e.call_method(obj,"load".into(),Vec::with_capacity(0))
}
#[inline]
pub fn locate_extension(e:&Executor,name:ZString)->Result<Value> {
    let obj: GcObject= match e.system.member("path") {
        Some(a)=>match a.try_into() {
            Ok(a)=>a,
            _=> return Err(Error::Custom("ImportError".to_string(),"Bad system.path".to_string()))
        },
        _=>return Err(Error::Custom("ImportError".to_string(),"No system.path".to_string()))
    };
    let len = obj.vec_len()?;
    if len == 0 {
        return Err(Error::Custom("ImportError".to_string(),"No paths on system.path".to_string()))
    }
    for i in 0..len {
        match obj.index_access(i){
            Some(a)=>{
                let a = a.invoke_to_string(&e);
                match import_protocol(&e,std::path::Path::new(a.as_ref()), name.clone())?{
                    Some(mods)=>{
                        return Ok(mods);
                    },
                    _=>{}
                }
            }
            _=>return Err(Error::Custom("ImportError".to_string(),"Bad system.path".to_string()))
        }
    }
    Err(Error::Custom("ImportError".to_string(),format!("Could not find module {}",name.as_ref())))
}
pub fn locate(e:&Executor,name:ZString)->Result<Value> {
    if let Some(_) = import_preload_builtins_spec(e, name.clone()) {
        return Ok(bultin_loader(name).into())
    }
    locate_extension(e, name.clone())
}

#[inline]
pub fn import_wrapper(e:Executor,args:Vec<Value>)-> Result<Value> {
    match args.len() {
        0=>Err(Error::InvalidArgs(1,0)),
        _=>{
            let name = args[0].invoke_to_string(&e);
            import(&e,name)
        }
    }
}
#[inline]
pub fn locate_wrapper(e:Executor,args:Vec<Value>)-> Result<Value> {
    match args.len() {
        0=>Err(Error::InvalidArgs(1,0)),
        _=>{
            let name = args[0].invoke_to_string(&e);
            locate(&e,name)
        }
    }
}