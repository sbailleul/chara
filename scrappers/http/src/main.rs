use clap::Parser;
use core::fmt;
use reqwest::{Client, Method, Request, Url};
use std::{fmt::Formatter, str::FromStr};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    url: String,
    #[arg(short, long, default_value = "GET")]
    method: String,
}

struct ClientArg {
    method: Method,
    url: Url,
}

enum ParseArgsError {
    ParseMethod(String),
    ParseUrl(String),
}
impl fmt::Display for ParseArgsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseArgsError::ParseMethod(method) => write!(f, "[Method : {method}] isn't valid"),
            ParseArgsError::ParseUrl(url) => write!(f, "[Url : {url}] isn't valid"),
        }
    }
}

impl Args {
    pub fn into_client(self) -> Result<ClientArg, ParseArgsError> {
        if let Ok(method) = Method::from_str(&self.method) {
            if let Ok(url) = Url::from_str(&self.url) {
                Ok(ClientArg { method, url })
            } else {
                Err(ParseArgsError::ParseUrl(self.url.clone()))
            }
        } else {
            Err(ParseArgsError::ParseMethod(self.method.clone()))
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    match args.into_client() {
        Ok(args) => {
            let client = Client::new();
            let resp = client
                .execute(Request::new(args.method, args.url))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            println!("{resp}");
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}
