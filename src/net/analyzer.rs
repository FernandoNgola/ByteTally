use crate::core::tracker::Tracker;
use std::io::{self, BufRead};

/// Lê linhas `IP BYTES`. Ignora linhas vazias e comentários iniciados por `#`.
/// Cada linha atribui bytes a um único IP; não representa um pacote PCAP.
pub fn analyze(reader: impl BufRead) -> io::Result<Tracker> {
    let mut tracker = Tracker::default();
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        let line = line.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        let invalid = |message: &str| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("linha {}: {message}", index + 1),
            )
        };
        let mut fields = line.split_whitespace();
        let ip = fields
            .next()
            .unwrap_or_default()
            .parse()
            .map_err(|_| invalid("endereço IP inválido"))?;
        let bytes_text = fields.next().ok_or_else(|| invalid("esperado: IP BYTES"))?;
        if !bytes_text.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(invalid("bytes devem ser um inteiro não negativo"));
        }
        let bytes = bytes_text
            .parse::<u64>()
            .map_err(|_| invalid("bytes fora do intervalo u64"))?;
        if fields.next().is_some() {
            return Err(invalid("esperado exactamente: IP BYTES"));
        }
        tracker.record(ip, bytes).map_err(invalid)?;
    }
    Ok(tracker)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_ipv4_ipv6_comments_and_empty_lines() {
        let tracker = analyze(&b"# exemplo\n\n::1 20\n192.168.0.1 10 # nota\n::1 5\n"[..]).unwrap();
        assert_eq!(
            tracker.ranked(),
            vec![
                ("::1".parse().unwrap(), 25),
                ("192.168.0.1".parse().unwrap(), 10)
            ]
        );
        assert!(analyze(&b""[..]).unwrap().ranked().is_empty());
    }

    #[test]
    fn rejects_bad_records_with_line_number() {
        for record in [
            "invalid 1",
            "::1",
            "::1 -1",
            "::1 1 extra",
            "::1 18446744073709551616",
        ] {
            let input = format!("# comment\n{record}\n");
            assert!(
                analyze(input.as_bytes())
                    .unwrap_err()
                    .to_string()
                    .starts_with("linha 2:")
            );
        }
        assert!(analyze(&b"::1 18446744073709551615\n::1 1"[..]).is_err());
    }
}
