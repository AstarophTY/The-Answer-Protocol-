pub fn ok(payload: &str) -> String {
    format!("OK {}\n", payload)
}

pub fn err(msg: &str) -> String {
    format!("ERR {}\n", msg)
}
