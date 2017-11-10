use std::collections::HashMap;
use std::sync::Mutex;
use std::process::exit;

lazy_static! {
    pub static ref SCRIPTS: Mutex<HashMap<&'static str, Script>> = Mutex::new(HashMap::new());
}

type Script = fn(Vec<String>);

macro_rules! script {
    ($name:expr => |$($arg:ident),*| $body:expr) => {
        (*SCRIPTS).lock().unwrap().insert($name, |args: Vec<String>| {
            let mut arg_iter = args.into_iter();
            $(
                let $arg = arg_iter.next().unwrap_or_else(|| {
                    eprintln!(concat!(
                        "Error: missing argument \"",
                        stringify!($arg),
                        "\" for command ",
                        stringify!($name),
                        "!"
                    ));
                    exit(1);
                });
            )*
            $body
        }).map(|_| panic!("attempted to register the same script twice!"));
    };
    ($name:expr => $body:expr) => {
        (*SCRIPTS).lock().unwrap().insert($name, |_args: Vec<String>| {
            $body
        }).map(|_| panic!("attempted to register the same script twice!"));
    };
}

pub fn register_scripts() {
    include!(concat!(env!("OUT_DIR"), "/scripts.rs"));
}

pub fn run_script(name: String, args: Vec<String>) {
    register_scripts();
    (*SCRIPTS).lock().unwrap().get(name.as_str()).expect(
        &(name +
              ": could not find this command"),
    )(args);
}
