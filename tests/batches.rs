use deleuze::{
    AppendOutcome, BatchOptions, Completion, Content, Error, Log, Options, Query, Value,
};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::time::Duration;

static ID: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "deleuze-batches-{}-{}.dlz",
            std::process::id(),
            ID.fetch_add(1, Ordering::Relaxed)
        ));
        Self(path)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn explicit_batch_preserves_input_order_errors_and_clean_reopen() {
    for completion in [Completion::Written, Completion::Synced] {
        let temp = Temp::new();
        let options = Options {
            completion,
            max_content_bytes: 40,
            max_depth: 0,
            batch: BatchOptions {
                max_records: 2,
                ..BatchOptions::default()
            },
            ..Options::default()
        };
        let log = Log::open(&temp.0, options).unwrap();
        assert!(log.append_batch(vec![]).is_empty());
        let inputs = vec![
            Content::Bytes(vec![1]),
            Content::Bytes(vec![0; 100]),
            Content::Properties(vec![("nested".into(), Value::List(vec![]))]),
            Content::Properties(vec![]),
        ];
        let mut results = log.append_batch(inputs.clone()).into_iter();
        assert_eq!(results.next().unwrap().unwrap().seq(), 1);
        let oversized = results.next().unwrap().unwrap_err();
        assert!(matches!(oversized.error, Error::TooLarge));
        assert_eq!(oversized.content, inputs[1]);
        assert_eq!(oversized.outcome, AppendOutcome::NotWritten);
        let deep = results.next().unwrap().unwrap_err();
        assert!(matches!(deep.error, Error::TooDeep));
        assert_eq!(deep.content, inputs[2]);
        assert_eq!(results.next().unwrap().unwrap().seq(), 2);
        drop(log);
        let log = Log::open(&temp.0, options).unwrap();
        assert_eq!(
            log.read(1).unwrap().unwrap().properties,
            vec![("content".into(), Value::Bytes(vec![1]))]
        );
        assert!(log.read(2).unwrap().unwrap().properties.is_empty());
        assert_eq!(log.append(Content::Properties(vec![])).unwrap().seq(), 3);
    }
}

#[test]
fn valid_record_above_batch_target_travels_alone() {
    for completion in [Completion::Written, Completion::Synced] {
        let temp = Temp::new();
        let log = Log::open(
            &temp.0,
            Options {
                completion,
                batch: BatchOptions {
                    max_records: 2,
                    max_bytes: 1,
                    ..BatchOptions::default()
                },
                ..Options::default()
            },
        )
        .unwrap();
        let results = log.append_batch((0..7).map(|n| Content::Bytes(vec![n; 200])).collect());
        assert_eq!(
            results
                .iter()
                .map(|r| r.as_ref().unwrap().seq())
                .collect::<Vec<_>>(),
            (1..=7).collect::<Vec<_>>()
        );
        for seq in 1..=7 {
            let entry = log.read(seq).unwrap().unwrap();
            assert_eq!(
                entry.properties,
                vec![("content".into(), Value::Bytes(vec![(seq - 1) as u8; 200]))]
            );
            assert_eq!(
                entry.record.property_hash(),
                results[(seq - 1) as usize]
                    .as_ref()
                    .unwrap()
                    .property_hash()
            );
        }
    }
}

#[test]
fn pending_byte_rejection_is_not_written_and_returns_content() {
    let temp = Temp::new();
    let log = Log::open(
        &temp.0,
        Options {
            completion: Completion::Synced,
            batch: BatchOptions {
                max_pending_bytes: 80,
                ..BatchOptions::default()
            },
            ..Options::default()
        },
    )
    .unwrap();
    let input = Content::Bytes(vec![9]);
    let error = log.append(input.clone()).unwrap_err();
    assert!(matches!(error.error, Error::QueueFull));
    assert_eq!(error.outcome, AppendOutcome::NotWritten);
    assert_eq!(error.content, input);
    assert!(log.query(Query::default()).unwrap().entries.is_empty());
}

#[test]
fn concurrent_synced_calls_keep_receipts_and_release_worker_file_lock_on_drop() {
    let temp = Temp::new();
    let log = Arc::new(
        Log::open(
            &temp.0,
            Options {
                completion: Completion::Synced,
                batch: BatchOptions {
                    max_records: 16,
                    max_delay: Duration::from_millis(3),
                    ..BatchOptions::default()
                },
                ..Options::default()
            },
        )
        .unwrap(),
    );
    let barrier = Arc::new(Barrier::new(17));
    let mut threads = Vec::new();
    for id in 0..16_u8 {
        let log = Arc::clone(&log);
        let barrier = Arc::clone(&barrier);
        threads.push(std::thread::spawn(move || {
            barrier.wait();
            (id, log.append(vec![id]).unwrap())
        }));
    }
    barrier.wait();
    let receipts: Vec<_> = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .collect();
    let mut seqs = receipts
        .iter()
        .map(|(_, record)| record.seq())
        .collect::<Vec<_>>();
    seqs.sort_unstable();
    assert_eq!(seqs, (1..=16).collect::<Vec<_>>());
    for (id, record) in receipts {
        let entry = log.read(record.seq()).unwrap().unwrap();
        assert_eq!(
            entry.properties,
            vec![("content".into(), Value::Bytes(vec![id]))]
        );
        assert_eq!(entry.record, record);
    }
    drop(log);
    let reopened = Log::open(&temp.0, Options::default()).unwrap();
    assert_eq!(reopened.append(vec![17]).unwrap().seq(), 17);
}

#[test]
fn zero_delay_synced_and_chunked_single_producer_batches_work() {
    let temp = Temp::new();
    let log = Log::open(
        &temp.0,
        Options {
            completion: Completion::Synced,
            batch: BatchOptions {
                max_records: 3,
                max_delay: Duration::ZERO,
                ..BatchOptions::default()
            },
            ..Options::default()
        },
    )
    .unwrap();
    let results = log.append_batch((0..8).map(|id| Content::Bytes(vec![id])).collect());
    assert_eq!(results.len(), 8);
    for (index, result) in results.into_iter().enumerate() {
        assert_eq!(result.unwrap().seq(), index as u64 + 1);
    }
}
