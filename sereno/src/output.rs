use std::io::{self, Write};

use clap::ValueEnum;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

pub fn print_text(text: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    write_text(&mut stdout, text)
}

pub fn print_text_error(text: &str) -> io::Result<()> {
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    write_text(&mut stderr, text)
}

pub fn print_json<T>(payload: &T) -> anyhow::Result<()>
where
    T: Serialize,
{
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    write_json(
        &mut stdout,
        &SuccessOutput {
            success: true,
            data: payload,
        },
    )?;
    Ok(())
}

pub fn print_json_error(error: &anyhow::Error) -> anyhow::Result<()> {
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    write_json(
        &mut stderr,
        &ErrorOutput {
            success: false,
            error: &error.to_string(),
        },
    )?;
    Ok(())
}

fn write_text<W>(writer: &mut W, text: &str) -> io::Result<()>
where
    W: Write,
{
    writeln!(writer, "{text}")
}

fn write_json<W, T>(writer: &mut W, payload: &T) -> serde_json::Result<()>
where
    W: Write,
    T: Serialize + ?Sized,
{
    serde_json::to_writer(&mut *writer, payload)?;
    writer.write_all(b"\n").map_err(serde_json::Error::io)
}

#[derive(Serialize)]
struct SuccessOutput<'a, T: ?Sized> {
    success: bool,
    data: &'a T,
}

#[derive(Serialize)]
struct ErrorOutput<'a> {
    success: bool,
    error: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(ErrorKind::BrokenPipe, "pipe closed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn text_output_propagates_writer_error() {
        let error = write_text(&mut FailingWriter, "text").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::BrokenPipe);
    }

    #[test]
    fn json_output_propagates_writer_error() {
        let error = write_json(
            &mut FailingWriter,
            &SuccessOutput {
                success: true,
                data: &"payload",
            },
        )
        .unwrap_err();
        assert!(error.is_io());
    }

    #[test]
    fn json_error_output_propagates_writer_error() {
        let error = write_json(
            &mut FailingWriter,
            &ErrorOutput {
                success: false,
                error: "failure",
            },
        )
        .unwrap_err();
        assert!(error.is_io());
    }
}
