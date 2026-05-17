use std::env;

pub fn get_env_vars() -> String {
    let mut vars: Vec<_> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));
    
    vars.into_iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<_>>()
        .join("\n")
}
