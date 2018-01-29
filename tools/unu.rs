/// Unu is a tool for extracting fenced code blocks from Markdown documents.
///
///    unu
///    (verb) (-hia) pull out, withdraw, draw out, extract.

use std::fs::File;
use std::io;

fn fenced(s: &str) -> bool {
    match s.get(0..3) {
        Some("```") => true,
        Some("~~~") => true,
        _ => false,
    }
}

fn extract<I: io::BufRead, O: io::Write>(source: I, sink: &mut O) -> io::Result<()> {
    let mut in_block = false;
    for line in source.lines() {
        let mut line = line?;
        if fenced(&*line) {
            in_block = !in_block;
        } else if in_block {
            line.push('\n');
            sink.write_all(line.as_bytes())?;
        }
    }
    Ok(())
}

fn main() {
    let mut args = std::env::args_os().peekable();
    let cmd = args.next().expect("No command name argument");
    if let Some(_) = args.peek() {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for fname in args {
            extract(
                io::BufReader::new(File::open(fname).expect("Can't open file")),
                &mut handle,
            ).expect("Can't output");
        }
    } else {
        println!(
            "unu\n(c) 2013-2017 charles childers\n\nTry:\n  {} filename",
            cmd.to_string_lossy()
        );
    }
}
