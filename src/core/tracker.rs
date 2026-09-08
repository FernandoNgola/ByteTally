use std::{collections::BTreeMap, net::IpAddr};

/// Totais em bytes por IP. Uma actualização nunca permite overflow.
#[derive(Debug, Default)]
pub struct Tracker {
    totals: BTreeMap<IpAddr, u64>,
}

impl Tracker {
    /// Acrescenta bytes; em caso de overflow preserva o total anterior.
    pub fn record(&mut self, ip: IpAddr, bytes: u64) -> Result<(), &'static str> {
        let total = self.totals.entry(ip).or_default();
        *total = total
            .checked_add(bytes)
            .ok_or("total de bytes excede u64")?;
        Ok(())
    }

    /// Ordena por consumo decrescente, com desempate por endereço IP.
    pub fn ranked(&self) -> Vec<(IpAddr, u64)> {
        let mut rows: Vec<_> = self
            .totals
            .iter()
            .map(|(&ip, &bytes)| (ip, bytes))
            .collect();
        rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_and_sorts_with_deterministic_ties() {
        let mut tracker = Tracker::default();
        let a = "192.168.0.1".parse().unwrap();
        let b = "192.168.0.2".parse().unwrap();
        tracker.record(b, 30).unwrap();
        tracker.record(a, 10).unwrap();
        tracker.record(a, 20).unwrap();
        assert_eq!(tracker.ranked(), vec![(a, 30), (b, 30)]);
    }

    #[test]
    fn overflow_preserves_previous_total() {
        let mut tracker = Tracker::default();
        let ip = "::1".parse().unwrap();
        tracker.record(ip, u64::MAX).unwrap();
        assert!(tracker.record(ip, 1).is_err());
        assert_eq!(tracker.ranked(), vec![(ip, u64::MAX)]);
    }
}
