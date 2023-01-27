use clap::Parser as ClapParser;
use obs2scrap::scrapbox;
use pulldown_cmark::Parser;
use std::{fs, path};

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Input Markdown file path
  #[arg(short, long)]
  file: String,
  /// Output file path. If not specified, output to stdout.
  #[arg(short, long)]
  output: Option<String>,
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  // Check args
  let file_path = path::Path::new(&args.file);
  if !file_path.exists() {
    anyhow::bail!("{} does not exist", file_path.display());
  }
  if !file_path.is_file() {
    anyhow::bail!("{} is not a file", file_path.display());
  }

  let output_path = match &args.output {
    Some(pathname) => {
      let output_path = path::Path::new(pathname);
      Some(output_path)
    }
    None => None,
  };

  // Convert Markdown to Scrapbox
  let md_content = fs::read_to_string(file_path)?;
  let parser = Parser::new(&md_content);
  let options = scrapbox::option::ScrapboxOption::default();
  let mut output = String::new();
  scrapbox::push_scrapbox(&mut output, parser, options);

  // Write output
  if let Some(path) = output_path {
    if path.exists() {
      if !path.is_file() {
        anyhow::bail!("{} is not a file", path.display());
      }
    } else {
      fs::File::create(path)?;
    }

    fs::write(path, output.as_bytes())?;
  } else {
    print!("{output}");
  }

  Ok(())
}
