#[macro_export]
use std::env;

// Takes a list of possible env vars and makes matching struct
// Env variables can be placed in the a struct with EnvVars::new()
//
// For instance:
//    collect_env![APP_NAME, APP_ADDRESS, APP_PORT];
//    fn main() {
//      vars = EnvVars::read();
//      println!("{}", vars.APP_NAME);
//    }

macro_rules! collect_env {
    ($($env_var: ident ),*) => {
        struct EnvVars {
            $($env_var: Option<String>),*
        }

        impl EnvVars {
            fn read() -> EnvVars {
            $(let $env_var = match env::var(stringify!($env_var)) {
              Ok(value) => Some(value),
              Err(_)    => None,
            };)*
            EnvVars {$($env_var),*}
            }
        }
    }
}
