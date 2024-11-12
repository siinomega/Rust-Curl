use reqwest::Client;
use clap::*;
use colored::Colorize;
use tokio;
use std::fs::File;
use std::io::Write;
use http::Method;
use http::Version;


// Args -> (Will Be Updated Regularly)
#[derive(Parser, Debug)]
#[command(about, long_about = None, disable_version_flag(true))]
struct Args {
    #[arg(short = 'u', long, help = "URL To Send The Request To")]
    url: String,

    #[arg(short = 'a', long = "user-agent", help = "Edit User-Agent String For The Request")]
    user_agent: Option<String>,

    #[arg(short = 's', long, help = "Save Result To Text File")]
    save: Option<String>,

    #[arg(short = 'd', long, help = "Additional Data To Include In The Request")]
    data: Option<String>,

    #[arg(short = 'x', long, help = "HTTP Method To Use For The Request")]
    method: Option<String>,

    #[arg(short = 'v', long = "version", help = "Additional Header To Include In The Request")]
    version: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.url.starts_with("http://") || args.url.starts_with("https://") {
        let client = Client::new();
        let mut request = client.get(&args.url);

        if let Some(user_agent) = &args.user_agent {
            request = request.header("User-Agent", user_agent);
        }
        if let Some(data) = &args.data {
            request = client.post(&args.url).body(data.clone());
        }
        if let Some(version) = &args.version {

            request = match version.as_str() {
                "1.1" => request.version(Version::HTTP_11),
                "2.0" => request.version(Version::HTTP_2),
                "1" => request.version(Version::HTTP_11),
                "2" => request.version(Version::HTTP_2),
                _ => {
                    eprintln!("{}", "Invalid HTTP Version ! Only (1.1 or 2.0) Are Allowed".red());
                    return;
                }
            };
        }

        if let Some(method) = &args.method {

            let method = Method::from_bytes(method.to_uppercase().as_bytes()).expect("Invalid HTTP Method !");
            request = match method {
                Method::POST => client.post(&args.url),
                Method::PUT => client.put(&args.url),
                Method::HEAD => client.head(&args.url),
                Method::OPTIONS => client.request(Method::OPTIONS, &args.url),
                Method::TRACE => client.request(Method::TRACE, &args.url),
                Method::CONNECT => client.request(Method::CONNECT, &args.url),
                Method::PATCH => client.patch(&args.url),
                Method::DELETE => client.delete(&args.url),
                Method::GET => client.get(&args.url),
                _ => client.get(&args.url),
            };
        }

        match request.send().await {
            Ok(response) => {
                let text = response.text().await.unwrap_or_else(|_| "Failed to read response body".red().parse().unwrap());
                println!("{}", text);
                if let Some(filename) = &args.save {

                    let filename = format!("{}.txt", filename);
                    if let Ok(mut file) = File::create(&filename) {
                        if let Err(err) = file.write_all(text.as_bytes()) {
                            eprintln!("{}", format!("Failed To Write To File: {}", err).red());
                        } else {
                            println!("{}", format!("Results Saved To {} !", filename).green());
                        }
                    } else {
                        eprintln!("{}", "Failed To Create File.".red());
                    }
                }
            }
            Err(err) => eprintln!("{}", format!("Request Failed! : {}", err).red()),
        }
    }
}