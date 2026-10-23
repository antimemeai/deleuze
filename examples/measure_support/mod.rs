//! Small std-only helpers shared by measurement examples, not production storage.
#![allow(dead_code)] // Each runner uses a different subset of this shared module.

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub const SAMPLE_HEADER: &str = "id\tdue_ns\tstart_ns\tend_ns\toutcome\tseq\thash";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Retained,
    Failed,
    Unknown,
    NotSent,
}
impl Outcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Retained => "retained",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
            Self::NotSent => "not_sent",
        }
    }
    fn parse(value: &str) -> io::Result<Self> {
        match value {
            "retained" => Ok(Self::Retained),
            "failed" => Ok(Self::Failed),
            "unknown" => Ok(Self::Unknown),
            "not_sent" => Ok(Self::NotSent),
            _ => Err(invalid("invalid sample outcome")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    pub id: u64,
    pub due_ns: u64,
    pub start_ns: u64,
    pub end_ns: u64,
    pub outcome: Outcome,
    pub seq: Option<u64>,
    pub hash: Option<[u8; 32]>,
}
impl Sample {
    pub fn service_ns(&self) -> u64 {
        self.end_ns - self.start_ns
    }
    pub fn scheduled_ns(&self) -> u64 {
        self.end_ns - self.due_ns
    }
    pub fn lag_ns(&self) -> u64 {
        self.start_ns - self.due_ns
    }
    fn validate(&self) -> io::Result<()> {
        if self.start_ns < self.due_ns || self.end_ns < self.start_ns {
            return Err(invalid("sample times must satisfy due <= start <= end"));
        }
        Ok(())
    }
}
fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn create(path: &Path) -> io::Result<BufWriter<File>> {
    Ok(BufWriter::new(
        OpenOptions::new().write(true).create_new(true).open(path)?,
    ))
}
pub fn hash_hex(hash: &[u8; 32]) -> String {
    hash.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn parse_hash(value: &str) -> io::Result<[u8; 32]> {
    if value.len() != 64 || !value.is_ascii() {
        return Err(invalid("sample hash must have 64 hex digits"));
    }
    let mut hash = [0; 32];
    for (index, byte) in hash.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16)
            .map_err(|_| invalid("invalid sample hash"))?;
    }
    Ok(hash)
}
pub fn write_samples(path: impl AsRef<Path>, samples: &[Sample]) -> io::Result<()> {
    let mut output = create(path.as_ref())?;
    writeln!(output, "{SAMPLE_HEADER}")?;
    for sample in samples {
        sample.validate()?;
        writeln!(
            output,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            sample.id,
            sample.due_ns,
            sample.start_ns,
            sample.end_ns,
            sample.outcome.as_str(),
            sample.seq.map(|n| n.to_string()).unwrap_or_default(),
            sample.hash.as_ref().map(hash_hex).unwrap_or_default()
        )?;
    }
    output.flush()
}
pub fn read_samples(path: impl AsRef<Path>) -> io::Result<Vec<Sample>> {
    let mut lines = BufReader::new(File::open(path)?).lines();
    if lines.next().transpose()?.as_deref() != Some(SAMPLE_HEADER) {
        return Err(invalid("unexpected sample TSV header"));
    }
    let mut samples = Vec::new();
    for line in lines {
        let line = line?;
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 7 {
            return Err(invalid("sample TSV needs seven fields"));
        }
        let number = |index: usize| {
            fields[index]
                .parse::<u64>()
                .map_err(|_| invalid("invalid sample number"))
        };
        let sample = Sample {
            id: number(0)?,
            due_ns: number(1)?,
            start_ns: number(2)?,
            end_ns: number(3)?,
            outcome: Outcome::parse(fields[4])?,
            seq: if fields[5].is_empty() {
                None
            } else {
                Some(number(5)?)
            },
            hash: if fields[6].is_empty() {
                None
            } else {
                Some(parse_hash(fields[6])?)
            },
        };
        sample.validate()?;
        samples.push(sample);
    }
    Ok(samples)
}
/// Nearest rank: sorted[ceil(percentile * N / 100) - 1]. No interpolation.
pub fn nearest_rank(values: &[u64], percentile: u32) -> Option<u64> {
    if values.is_empty() || !(1..=100).contains(&percentile) {
        return None;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let rank = (sorted.len() as u128 * u128::from(percentile)).div_ceil(100) as usize;
    Some(sorted[rank - 1])
}
pub fn make_payload(id: u64, bytes: usize) -> io::Result<Vec<u8>> {
    if bytes < 8 {
        return Err(invalid("payload size must be at least eight"));
    }
    let mut payload = Vec::new();
    payload.try_reserve_exact(bytes).map_err(io::Error::other)?;
    payload.resize(bytes, 0xa5);
    payload[..8].copy_from_slice(&id.to_le_bytes());
    Ok(payload)
}
pub fn verify_payload(payload: &[u8], expected_bytes: usize) -> io::Result<u64> {
    if payload.len() != expected_bytes || expected_bytes < 8 {
        return Err(invalid("unexpected payload size"));
    }
    if payload[8..].iter().any(|byte| *byte != 0xa5) {
        return Err(invalid("payload pattern differs"));
    }
    Ok(u64::from_le_bytes(
        payload[..8].try_into().expect("eight-byte ID"),
    ))
}
pub fn write_summary(path: impl AsRef<Path>, fields: &[(&str, String)]) -> io::Result<()> {
    let mut output = create(path.as_ref())?;
    writeln!(output, "metric\tvalue")?;
    for (key, value) in fields {
        if key.contains(['\t', '\n', '\r']) || value.contains(['\t', '\n', '\r']) {
            return Err(invalid("summary fields cannot contain tabs/newlines"));
        }
        writeln!(output, "{key}\t{value}")?;
    }
    output.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nearest_rank_uses_individual_samples_not_interpolation() {
        let values = [90, 1, 20, 5];
        assert_eq!(nearest_rank(&values, 50), Some(5));
        assert_eq!(nearest_rank(&values, 95), Some(90));
        assert_eq!(nearest_rank(&[17], 99), Some(17));
        assert_eq!(nearest_rank(&[], 50), None);
        assert_eq!(nearest_rank(&values, 0), None);
    }
    #[test]
    fn pattern_verification_detects_length_and_content_changes() {
        let mut payload = make_payload(9, 16).unwrap();
        assert_eq!(verify_payload(&payload, 16).unwrap(), 9);
        assert!(verify_payload(&payload, 15).is_err());
        payload[12] = 0;
        assert!(verify_payload(&payload, 16).is_err());
    }
    #[test]
    fn sample_tsv_roundtrips_absent_receipts_and_rejects_overwrite() {
        let path = std::env::temp_dir().join(format!(
            "deleuze-samples-{}-{}.tsv",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let samples = vec![
            Sample {
                id: 1,
                due_ns: 10,
                start_ns: 15,
                end_ns: 20,
                outcome: Outcome::Retained,
                seq: Some(3),
                hash: Some([255; 32]),
            },
            Sample {
                id: 2,
                due_ns: 20,
                start_ns: 20,
                end_ns: 20,
                outcome: Outcome::NotSent,
                seq: None,
                hash: None,
            },
        ];
        write_samples(&path, &samples).unwrap();
        assert_eq!(read_samples(&path).unwrap(), samples);
        assert!(write_samples(&path, &samples).is_err());
        std::fs::remove_file(path).unwrap();
    }
}
