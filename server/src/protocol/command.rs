/// All commands the server understands. Add variants here as the protocol grows.
#[derive(Debug)]
pub enum Command {
    Connect { name: String },
    Who,
    Look,
    Unknown(String),
}

impl Command {
    pub fn parse(line: &str) -> Result<Self, String> {
        let mut parts = line.splitn(2, ' ');
        let verb = parts.next().unwrap_or("").to_uppercase();
        let rest = parts.next().unwrap_or("").trim();

        match verb.as_str() {
            "CONNECT" => {
                if rest.is_empty() {
                    Err("CONNECT requires a name".to_string())
                } else {
                    Ok(Command::Connect { name: rest.to_string() })
                }
            }
            "WHO" => Ok(Command::Who),
            "LOOK" => Ok(Command::Look),
            other => Ok(Command::Unknown(other.to_string())),
        }
    }
}
