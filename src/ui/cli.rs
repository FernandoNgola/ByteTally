use crate::net::analyzer::analyze;
use std::{
    ffi::OsString,
    fs::File,
    io::{self, BufReader, Write},
};

const HELP: &str = "ByteTally — relatório de bytes por IP

Uso:
  bytetally analyze <ficheiro>
  bytetally --help
  bytetally --version

Formato: um registo IP BYTES por linha (IPv4 ou IPv6).
Linhas vazias e comentários com # são ignorados.
";

/// Executa a CLI com argumentos sem o nome do programa.
pub fn run(args: impl IntoIterator<Item = OsString>, mut output: impl Write) -> io::Result<()> {
    let args: Vec<_> = args.into_iter().collect();
    match args.as_slice() {
        [] => write!(output, "{HELP}"),
        [flag] if flag == "--help" || flag == "-h" => write!(output, "{HELP}"),
        [flag] if flag == "--version" || flag == "-V" => {
            writeln!(output, "bytetally {}", env!("CARGO_PKG_VERSION"))
        }
        [command, path] if command == "analyze" => {
            let file = File::open(path).map_err(|error| {
                io::Error::new(error.kind(), format!("{}: {error}", path.to_string_lossy()))
            })?;
            let rows = analyze(BufReader::new(file))?.ranked();
            if rows.is_empty() {
                return writeln!(output, "Nenhum registo de tráfego.");
            }
            writeln!(output, "IP\tBYTES")?;
            for (ip, bytes) in rows {
                writeln!(output, "{ip}\t{bytes}")?;
            }
            Ok(())
        }
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "argumentos inválidos; use bytetally --help",
        )),
    }
}
