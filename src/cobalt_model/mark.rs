use serde::Serialize;

use crate::error::*;

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MarkdownBuilder {
    pub args: Vec<String>,
}

impl MarkdownBuilder {
    pub fn build(self) -> Markdown {
        Markdown {
            args: self.args,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Markdown {
    args: Vec<String>,
}

impl Markdown {
    pub fn parse(&self, content: &str) -> Result<String> {
				use std::io::{Read, Write};
        use std::process::{Command, Stdio};

        let mut child = Command::new("/bin/mmark")
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        {
          let mut stdin = child.stdin.take().unwrap();
          stdin.write_all(content.as_bytes()).unwrap();
          // EOF gets sent here.
        }

        let stdout = child.stdout.as_mut().unwrap();
        let mut output = String::new();
        stdout.read_to_string(&mut output).unwrap();

        if !child.wait().unwrap().success() {
            failure::bail!("mmark error:\n{}", &output);
        }

        Ok(output)
    }
}
