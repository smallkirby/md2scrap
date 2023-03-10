use std::io;

use pulldown_cmark::escape::StrWrite;
use pulldown_cmark::{
  CodeBlockKind,
  Event::{self, *},
  LinkType, Tag,
};

mod list;
pub mod option;

use option::ScrapboxOption;

struct ScrapboxWriter<I, W> {
  iter: I,
  writer: W,
  runtime: ParseRuntime,
  option: ScrapboxOption,
}

struct ParseRuntime {
  pub list: list::ListHandler,
  pub is_in_codeblock: bool,
}

impl ParseRuntime {
  fn new() -> Self {
    Self {
      list: list::ListHandler::new(),
      is_in_codeblock: false,
    }
  }
}

impl<'a, I, W> ScrapboxWriter<I, W>
where
  I: Iterator<Item = Event<'a>>,
  W: StrWrite,
{
  fn new(iter: I, writer: W, option: ScrapboxOption) -> Self {
    Self {
      iter,
      writer,
      runtime: ParseRuntime::new(),
      option,
    }
  }

  fn run(mut self) -> io::Result<()> {
    while let Some(event) = self.iter.next() {
      match event {
        Text(text) => {
          if self.runtime.is_in_codeblock {
            let lines = text.split('\n');
            for line in lines {
              self.writeln(&format!(" {line}"))?;
            }
          } else {
            self.write(&text)?;
          }
        }
        Start(tag) => {
          self.start_tag(tag)?;
        }
        End(tag) => {
          self.end_tag(tag)?;
        }
        Code(code) => {
          self.write(&format!("`{code}`"))?;
        }
        _ => {}
      }
    }

    Ok(())
  }

  #[inline]
  fn write(&mut self, s: &str) -> io::Result<()> {
    self.writer.write_str(s)?;
    Ok(())
  }

  #[inline]
  fn writeln(&mut self, s: &str) -> io::Result<()> {
    self.writer.write_str(s)?;
    self.writer.write_str("\n")?;
    Ok(())
  }

  fn start_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
    match tag {
      Tag::Heading(level, _, _) => {
        let heading_level = if level as usize > 6 {
          6
        } else {
          level as usize
        };
        if self.option.newline_before_heading {
          self.writeln("")?;
        }
        self.write(&format!("[{} ", "*".repeat(6 - heading_level + 1)))?; // TODO
      }
      Tag::List(order) => {
        let s = self.runtime.list.start_list(order);
        self.write(&s)?;
      }
      Tag::Item => {
        let s = self.runtime.list.get();
        self.write(&s)?;
      }
      Tag::Strong => {
        self.write("[* ")?;
      }
      Tag::Strikethrough => {
        self.write("[- ")?;
      }
      Tag::Emphasis => {
        self.write("[/ ")?;
      }
      Tag::Link(LinkType::Inline, url, _) => {
        self.write(&format!("[{url} "))?;
      }
      Tag::CodeBlock(kind) => {
        match kind {
          CodeBlockKind::Indented => {
            self.writeln("code: unnamed")?;
          }
          CodeBlockKind::Fenced(prog) => {
            let name = if prog.len() > 0 { &prog } else { "unnamed" };
            self.writeln(&format!("code:{name}"))?;
          }
        }

        self.runtime.is_in_codeblock = true;
      }
      _ => {}
    }

    Ok(())
  }

  fn end_tag(&mut self, tag: Tag<'a>) -> io::Result<()> {
    match tag {
      Tag::Heading(_, _, _) => {
        self.writeln("]")?;
      }
      Tag::Paragraph => {
        self.writeln("")?;
      }
      Tag::List(_) => {
        self.runtime.list.end_list();
      }
      Tag::Item => {
        let s = if self.runtime.list.depth() <= 1 {
          "\n"
        } else {
          ""
        };
        self.write(s)?;
      }
      Tag::Strong | Tag::Strikethrough | Tag::Emphasis => {
        self.write("]")?;
      }
      Tag::Link(LinkType::Inline, _, _) => {
        self.write("]")?;
      }
      Tag::CodeBlock(_) => {
        self.runtime.is_in_codeblock = false;
      }
      _ => {}
    }

    Ok(())
  }
}

pub fn push_scrapbox<'a, I>(s: &mut String, iter: I, option: ScrapboxOption)
where
  I: Iterator<Item = Event<'a>>,
{
  ScrapboxWriter::new(iter, s, option).run().unwrap();
}
