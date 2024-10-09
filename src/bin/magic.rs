use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    test: Option<String>,
}

fn main() {
  let args = Args::parse();

	println!("abracadabra {}!", args.test.unwrap_or(String::from("")));
}
