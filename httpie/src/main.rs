use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// 定义 HTTPPie 的 CLI 的主入口，它包含若子命令
// 下面 ///  的注释是文档，clap 会将其作为 CLI 的帮助

/// A naive httpie implementation with Rust, can you imagine how easy it?
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Qing Chen <qing@chen.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Subcommand, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post), // TODO:  添加其他的 http 方法
}

/// feed get with an url and we will retrieve the reponse for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[arg(value_parser=parse_url)]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[arg(value_parser=parse_url)]
    url: String,
    /// HTTP 请求的 body
    #[arg(value_parser=parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splited = s.split('=');
        let err = || anyhow::anyhow!(format!("Failed to parse {}", s));

        Ok(Self {
            k: (splited.next().ok_or_else(err)?).to_string(),
            v: (splited.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;

    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();

    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    let resp = client.post(&args.url).json(&body).send().await?;

    Ok(print_resp(resp).await?)
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    print!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        print!("{}: {:?}", name.to_string().green(), value);
    }

    print!("\n");
}

fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),

        // 其它 mime type，我们就直接输出
        _ => println!("{}", body),
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);

    let mime = get_content_type(&resp);
    let body = resp.text().await?;

    print_body(mime, &body);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    let client = Client::builder().default_headers(headers).build()?;

    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}
