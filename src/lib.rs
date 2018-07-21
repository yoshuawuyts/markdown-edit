#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]

extern crate pulldown_cmark;
extern crate pulldown_cmark_to_cmark;
#[macro_use]
extern crate failure;

use pulldown_cmark::{Event, Parser, Tag};
use pulldown_cmark_to_cmark::fmt::cmark;
use std::{fs, path};

/// The Markdown object.
pub struct Markdown<'p> {
  parser: Parser<'p>,
}

impl<'p> Markdown<'p> {
  /// Create a new instance.
  #[inline]
  pub fn new(raw_md: &'p str) -> Self {
    Self {
      parser: Parser::new(raw_md),
    }
  }

  /// Replace the body for a header.
  pub fn replace_body(
    self,
    header: &str,
    section: Vec<Event>,
  ) -> Result<String, failure::Error> {
    // Define the internal state machine.
    let mut inspect_header = false;
    let mut header_found = false;
    let mut should_replace_section = false;
    let mut sections_replaced = false;

    let events: Vec<Event> = {
      // Remove the unused text.
      self
        .parser
        .into_iter()
        .flat_map(move |event| {
          // Find all headers.
          if let Event::Start(tag) = &event {
            if let Tag::Header(_text) = tag {
              inspect_header = true;
            }
          }

          // Read the header text.
          if inspect_header {
            inspect_header = false;
            if let Event::Text(text) = &event {
              if text == header {
                header_found = true;
              }
            }
          }

          // Edit the body.
          if should_replace_section {
            let mut should_continue = true;
            if let Event::Start(tag) = &event {
              if let Tag::Header(_) = tag {
                should_replace_section = false;
                should_continue = false;
              }
            }

            if should_continue {
              return if !sections_replaced {
                sections_replaced = true;
                // FIXME: heh, this is inefficient.
                section.clone()
              } else {
                vec![]
              };
            }
          }

          // Unless specified otherwise, return the event.
          vec![event]
        })
        .collect()
    };

    if sections_replaced {
      let mut buf = String::from("");
      cmark(events.iter(), &mut buf, None)?;
      Ok(buf)
    } else if header_found {
      bail!("No header body found to replace.");
    } else {
      bail!("Could not find header");
    }
  }
}

/// Replace
pub fn replace_body(
  path: path::PathBuf,
  header: &str,
  body: String,
) -> Result<(), failure::Error> {
  let target = fs::read_to_string(&path)?;
  let body: Vec<Event> = Parser::new(&body).into_iter().collect();
  let parser = Markdown::new(&target);
  let res = parser.replace_body(header, body)?;
  fs::write(&path, res)?;
  Ok(())
}
