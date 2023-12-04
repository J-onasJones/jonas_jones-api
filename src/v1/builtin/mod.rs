pub fn help() -> &'static str {
    return "Please refer to the wiki at https://wiki.jonasjones.dev/Api/"
}

pub fn ping() -> &'static str {
    return "pong"
}

pub fn version() -> &'static str {
    return option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
}
