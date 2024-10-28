use std::io::{stdout, stdin, Read, Write};
use promkit::preset::readline::Readline;
use clap::Parser;

use navaja::proxy::ZAP;
use navaja::http::HTTP;
use navaja::Result;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    parameter: String,
    #[arg(long)]
    proxy: Option<String>,

    target: String
}

fn pause(msg: &str) {
    let mut stdout = stdout();
    stdout.write(msg.as_bytes()).unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();

  let api = ZAP::new(
      "qa26kjre8bkihtg7q767cnnb2j",
      &args.target,
      &args.proxy.unwrap_or(String::from("http://localhost:8080")),
  );

  let req: HTTP;

  let mut prompt = Readline::default()
      .title("Entered SQL prompt!")
      .prompt()?;

  let mut sql: String;
  let save_dir = format!("{}/.aguja", std::env::var("HOME")?);

  if let Err(_) = std::fs::read_dir(&save_dir) {
      std::fs::create_dir(&save_dir)?;
  }

  api.add_break().await?;
  pause("Please send the target request and press Enter to continue...");

  req = api.get_http().await?;
  println!("{req:#?}");
  api.remove_break().await?;

  loop {
      sql = prompt.run()?;
      if sql.to_lowercase() != "exit" {
          api.inject_sql(&req, &args.parameter, &sql).await?;
      } else {
          break;
      }
  }

  Ok(())
}
