use crate::protocol::response::Response;

#[derive(Debug)]
pub enum Command {
    Connect { name: String },
    Who,
    Look,
    Group { action: String },
    Chat { text: String },
    Unknown(String),
}

impl Command {
    pub fn parse(line: &str) -> Result<Self, Response> {
        let mut parts: std::str::SplitN<'_, char> = line.splitn(2, ' ');
        let verb: String = parts.next().unwrap_or("").to_uppercase();
        let rest: &str = parts.next().unwrap_or("").trim();

        match verb.as_str() {
            "CONNECT" => {
                if rest.is_empty() {
                    Err(Response::error(400, "CONNECT requires a name"))
                } else {
                    Ok(Command::Connect { name: rest.to_string() })
                }
            }
            "WHO" => Ok(Command::Who),
            "LOOK" => Ok(Command::Look),
            "GROUP" => Ok(Command::Group { action: rest.to_string() }),
            "CHAT" => {
                if rest.is_empty() {
                    Err(Response::error(400, "CHAT requires a message"))
                } else {
                    Ok(Command::Chat { text: rest.to_string() })
                }
            }
            other => Ok(Command::Unknown(other.to_string())),
        }
    }
}
