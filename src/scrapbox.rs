use std::io;

use pulldown_cmark::escape::StrWrite;
use pulldown_cmark::{
  Event::{self, *},
  Tag,
};

mod list;

struct ScrapboxWriter<I, W> {
  iter: I,
  writer: W,
  runtime: ParseRuntime,
}

struct ParseRuntime {
  pub list: list::ListHandler,
}

impl ParseRuntime {
  fn new() -> Self {
    Self {
      list: list::ListHandler::new(),
    }
  }
}

impl<'a, I, W> ScrapboxWriter<I, W>
where
  I: Iterator<Item = Event<'a>>,
  W: StrWrite,
{
  fn new(iter: I, writer: W) -> Self {
    Self {
      iter,
      writer,
      runtime: ParseRuntime::new(),
    }
  }

  fn run(mut self) -> io::Result<()> {
    while let Some(event) = self.iter.next() {
      match event {
        Text(text) => {
          self.write(&text)?;
        }
        Start(tag) => {
          self.start_tag(tag)?;
        }
        End(tag) => {
          self.end_tag(tag)?;
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
      _ => {}
    }

    Ok(())
  }
}

pub fn push_scrapbox<'a, I>(s: &mut String, iter: I)
where
  I: Iterator<Item = Event<'a>>,
{
  ScrapboxWriter::new(iter, s).run().unwrap();
}
