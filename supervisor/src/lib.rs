use std::error::Error;
use std::path::Path;
use std::process::{Child, Command};
use std::{fmt, io};

#[derive(Debug, Clone)]
pub struct NodeJsError {
    cause: String,
    description: String,
}

impl NodeJsError {
    pub fn new(cause: String, description: String) -> Self {
        NodeJsError {
            cause,
            description,
        }
    }
}

impl fmt::Display for NodeJsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cause: {};\ndescription: {}", self.cause, self.description)
    }
}

impl Error for NodeJsError {
    fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug)]
pub struct NodeJsProc {
    child: Child,
    server: String
}

impl NodeJsProc {
    pub fn start(server_path: String) -> Result<Self, NodeJsError> {
        NodeJsProc::start_with_server_url(server_path, "localhost:15000".into())
    }

    pub fn start_with_server_url(server_path: String, server_url: String) -> Result<Self, NodeJsError> {
        let path = Path::new(&server_path);

        if !path.exists() {
            return Err(NodeJsError::new("Invalid path".into(), "'ssr.js' file not found in the given path.".into()))
        }

        let string_path = match path.to_str() {
            None => return Err(NodeJsError::new("Invalid path".into(), "The given path contains unvalid UTF-8 characters.".into())),
            Some(path) => path,
        };

        let child = match Command::new("node")
            .arg(string_path)
            .spawn() {
                Err(err) => return Err(NodeJsError::new(
                    "Process error".into(),
                    format!("Something went wrong on invoking a node server: {}", err.to_string())
                )),
                Ok(child) => child
            };

        Ok(NodeJsProc {
            child,
            server: server_url 
        })
    }

    pub async fn kill(self) -> io::Result<()> {
        let resp = reqwest::Client::new()
            .get(format!("{}/shutdown", self.server))
            .send()
            .await;

        if resp.is_err() {
            println!("Failed to call exit endpoint, killing process forcedly: {}", resp.unwrap_err());
            self.force_kill()?;
        }

        Ok(())
    }

    pub fn force_kill(mut self) -> io::Result<()> {
        self.child.kill()
    }
}
