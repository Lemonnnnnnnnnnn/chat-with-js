mod resolve;
mod utils;

use std::{fs::File, path::Path};

use clap::Parser;
use reqwest::Client;
use resolve::entry::resolve_entry;
use serde::Deserialize;
use serde_json::json;
use std::io::Write;

#[derive(Parser)]
struct Args {
    entry_file: String,
    task: String,
}

#[tokio::main] // 使用 tokio 作为异步运行时
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let entry_path = Path::new(&args.entry_file);

    // 解析文件内容
    let context = resolve_entry(entry_path)?;

    // 向 Ollama 服务器发送请求
    let response = send_to_ollama(&context, &args.task).await?;

    let mut file = File::create("llm-result.md").expect("无法创建文件");

    // 向文件中写入文本
    file.write_all(response.as_bytes()).expect("写入失败");

    Ok(())
}

#[derive(Deserialize)]
struct OllamaResult {
    response: String,
}

async fn send_to_ollama(context: &str, task: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://localhost:11434/api/generate"; // Ollama 服务器地址
    let prompt = format!("我有一些代码块如下：{},{}", context, task);
    let model = "codellama:7b-instruct";

    let body = json!({
        "prompt": prompt,
        "model": model,
        "stream" : false,
    });

    let response = client
        .post(url)
        .json(&body) // 发送 JSON 数据
        .send()
        .await? // 等待请求完成
        .json::<OllamaResult>()
        .await?; // 等待响应 body 的文本内容

    Ok(response.response)
}
