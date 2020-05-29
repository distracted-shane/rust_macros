// Takes a list of possible env vars and makes matching struct
// Env variables can be placed in the a struct with EnvVars::new()
//
// For instance:
//    collect_env![name=EnvVars; APP_NAME, APP_ADDRESS, APP_PORT];
//    fn main() {
//      vars = EnvVars::read();
//      println!("{}", vars.APP_NAME);
//    }

#[macro_export]
macro_rules! collect_env {
    (name=$_name: ident;$($env_var: ident ),*) => {
        struct $_name {
            $($env_var: Option<String>),*
        }

        impl $_name {
            fn read() -> $_name {
            $(let $env_var = match env::var(stringify!($env_var)) {
              Ok(value) => Some(value),
              Err(_)    => None,
            };)*
            $_name {$($env_var),*}
            }
        }
    }
}
