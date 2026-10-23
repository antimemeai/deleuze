use deleuze::{AppendOutcome, Content, Error, Log, Options, Properties, Query, Value};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};

static TEMP_ID: AtomicU64 = AtomicU64::new(0);
struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "deleuze-test-{}-{}",
            std::process::id(),
            TEMP_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn file(&self) -> PathBuf {
        self.0.join("events.dlz")
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

// Independent hand-assembled format fixture. Does not use the production packer
// or frame encoder. Primitive hashing is checked against an official vector below.
fn fixture(payload: &[u8], seq: u64) -> Vec<u8> {
    let mut bytes = b"DLZLOG01\x01\0\0\0DLZR".to_vec();
    bytes.extend((payload.len() as u64).to_le_bytes());
    bytes.extend(seq.to_le_bytes());
    bytes.extend((-2_i64).to_le_bytes());
    bytes.extend(123_u32.to_le_bytes());
    bytes.extend(blake3::hash(payload).as_bytes());
    bytes.extend(payload);
    bytes.extend(b"ENDR");
    bytes.extend((76 + payload.len() as u64).to_le_bytes());
    bytes
}
fn empty_fixture() -> Vec<u8> {
    fixture(&[0; 8], 1)
}
fn read_log(path: &std::path::Path) -> Log {
    Log::open(path, Options::default()).unwrap()
}

#[test]
fn official_blake3_empty_vector() {
    assert_eq!(
        blake3::hash(b"").to_hex().as_str(),
        "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
    );
}

#[test]
fn reads_hand_assembled_fixture_then_appends_after_clean_reopen() {
    let temp = Temp::new();
    let path = temp.file();
    let original = empty_fixture();
    fs::write(&path, &original).unwrap();
    let log = read_log(&path);
    let entry = log.read(1).unwrap().unwrap();
    assert!(entry.properties.is_empty());
    assert_eq!(entry.record.seq(), 1);
    assert_eq!(entry.record.created_at().unix_seconds(), -2);
    assert_eq!(entry.record.created_at().nanos(), 123);
    assert_eq!(
        entry.record.property_hash(),
        blake3::hash(&[0; 8]).as_bytes()
    );
    let record = log.append("existing output".to_owned()).unwrap();
    assert_eq!(record.seq(), 2);
    drop(log);
    assert_eq!(
        read_log(&path).read(2).unwrap().unwrap().properties,
        vec![("content".into(), Value::Text("existing output".into()))]
    );
    assert!(fs::read(path).unwrap().starts_with(&original));
}

#[test]
fn writer_matches_independent_property_and_frame_layout() {
    let temp = Temp::new();
    let path = temp.file();
    let log = read_log(&path);
    let properties = vec![
        ("x".into(), Value::U64(u64::MAX)),
        ("x".into(), Value::Null),
    ];
    let record = log.append(properties).unwrap();
    let mut payload = 2_u64.to_le_bytes().to_vec();
    payload.extend(1_u64.to_le_bytes());
    payload.extend(b"x");
    payload.push(3);
    payload.extend(u64::MAX.to_le_bytes());
    payload.extend(1_u64.to_le_bytes());
    payload.extend(b"x");
    payload.push(0);
    let mut expected = fixture(&payload, 1);
    expected[32..40].copy_from_slice(&record.created_at().unix_seconds().to_le_bytes());
    expected[40..44].copy_from_slice(&record.created_at().nanos().to_le_bytes());
    assert_eq!(fs::read(path).unwrap(), expected);
}

#[test]
fn preserves_values_duplicates_bits_and_producer_times_as_properties() {
    let temp = Temp::new();
    let log = read_log(&temp.file());
    let values = vec![
        Value::Null,
        Value::Bool(false),
        Value::Bool(true),
        Value::I64(i64::MIN),
        Value::U64(u64::MAX),
        Value::F32Bits(0x7fc0_0001),
        Value::F64Bits(0x8000_0000_0000_0000),
        Value::Text("λ".into()),
        Value::Bytes(vec![0xff, 0]),
        Value::List(vec![Value::Null]),
        Value::Properties(vec![("n".into(), Value::Bool(true))]),
        Value::Encoded {
            representation: "decimal".into(),
            bytes: b"1e999999".to_vec(),
        },
    ];
    let mut properties: Properties = values
        .into_iter()
        .map(|v| ("duplicate".into(), v))
        .collect();
    properties.push(("created_at".into(), Value::I64(i64::MIN)));
    properties.push(("seq".into(), Value::U64(999)));
    let first = log.append(properties.clone()).unwrap();
    let second = log.append(properties.clone()).unwrap();
    assert_eq!(first.property_hash(), second.property_hash());
    assert_eq!((first.seq(), second.seq()), (1, 2));
    assert_ne!(first.created_at().unix_seconds(), i64::MIN);
    let mut entry = log.read(1).unwrap().unwrap();
    assert_eq!(entry.properties, properties);
    entry.properties.clear();
    assert_eq!(log.read(1).unwrap().unwrap().properties, properties);
}

#[test]
fn empty_representations_and_opaque_bytes_are_distinct() {
    let temp = Temp::new();
    let log = read_log(&temp.file());
    let inputs = [
        Content::Text(String::new()),
        Content::Bytes(Vec::new()),
        Content::Properties(Vec::new()),
        Content::Encoded {
            representation: "invalid-utf8".into(),
            bytes: vec![0xff],
        },
    ];
    for input in inputs {
        log.append(input).unwrap();
    }
    let entries = log.query(Query::default()).unwrap().entries;
    assert_eq!(entries.len(), 4);
    assert_ne!(
        entries[0].record.property_hash(),
        entries[1].record.property_hash()
    );
    assert_ne!(
        entries[1].record.property_hash(),
        entries[2].record.property_hash()
    );
    assert_eq!(
        entries[3].properties,
        vec![(
            "content".into(),
            Value::Encoded {
                representation: "invalid-utf8".into(),
                bytes: vec![0xff]
            }
        )]
    );
}

#[test]
fn size_depth_failures_return_input_and_do_not_consume_sequence() {
    let temp = Temp::new();
    let options = Options {
        max_content_bytes: 64,
        max_depth: 0,
        ..Options::default()
    };
    let log = Log::open(temp.file(), options).unwrap();
    let large = Content::Bytes(vec![0; 65]);
    let error = log.append(large.clone()).unwrap_err();
    assert!(matches!(error.error, Error::TooLarge));
    assert_eq!(error.outcome, AppendOutcome::NotWritten);
    assert_eq!(error.content, large);
    let nested = Content::Properties(vec![("x".into(), Value::List(Vec::new()))]);
    let error = log.append(nested.clone()).unwrap_err();
    assert!(matches!(error.error, Error::TooDeep));
    assert_eq!(error.content, nested);
    let opaque = Content::Encoded {
        representation: "json".into(),
        bytes: b"[[[[[[[[".to_vec(),
    };
    assert_eq!(log.append(opaque).unwrap().seq(), 1);
}

#[test]
fn ingest_depth_does_not_reclassify_valid_history_as_corruption() {
    let temp = Temp::new();
    let path = temp.file();
    let properties = vec![(
        "x".into(),
        Value::List(vec![Value::List(vec![Value::Null])]),
    )];
    let log = read_log(&path);
    log.append(properties.clone()).unwrap();
    drop(log);
    let log = Log::open(
        &path,
        Options {
            max_depth: 0,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(log.read(1).unwrap().unwrap().properties, properties);
}

#[test]
fn reopen_capacity_error_is_not_corruption_or_file_mutation() {
    let temp = Temp::new();
    let path = temp.file();
    let log = read_log(&path);
    log.append(vec![0; 100]).unwrap();
    drop(log);
    let before = fs::read(&path).unwrap();
    assert!(matches!(
        Log::open(
            &path,
            Options {
                max_content_bytes: 8,
                ..Options::default()
            }
        ),
        Err(Error::ResourceLimit { seq: 1, .. })
    ));
    assert_eq!(fs::read(&path).unwrap(), before);
    assert!(read_log(&path).read(1).unwrap().is_some());
}

#[test]
fn corrupt_and_incomplete_files_are_rejected_unchanged() {
    let temp = Temp::new();
    let path = temp.file();
    let original = empty_fixture();
    let mut cases = vec![
        Vec::new(),
        original[..11].to_vec(),
        original[..original.len() - 1].to_vec(),
        original[..30].to_vec(),
    ];
    for index in [
        0,
        8,
        12,
        24,
        44,
        76,
        original.len() - 12,
        original.len() - 1,
    ] {
        let mut changed = original.clone();
        changed[index] ^= 0x80;
        cases.push(changed);
    }
    let mut huge = original.clone();
    huge[16..24].copy_from_slice(&u64::MAX.to_le_bytes());
    cases.push(huge);
    let mut bad_time = original.clone();
    bad_time[40..44].copy_from_slice(&1_000_000_000_u32.to_le_bytes());
    cases.push(bad_time);
    let mut extra = original.clone();
    extra.push(0);
    cases.push(extra);
    for bytes in cases {
        fs::write(&path, &bytes).unwrap();
        assert!(matches!(
            Log::open(&path, Options::default()),
            Err(Error::Corruption { .. })
        ));
        assert_eq!(fs::read(&path).unwrap(), bytes);
    }
}

#[test]
fn hash_correct_but_invalid_values_are_rejected() {
    let temp = Temp::new();
    let path = temp.file();
    for value in [
        &[1, 2][..],
        &[6, 1, 0, 0, 0, 0, 0, 0, 0, 0xff][..],
        &[255][..],
    ] {
        let mut payload = 1_u64.to_le_bytes().to_vec();
        payload.extend(0_u64.to_le_bytes());
        payload.extend(value);
        let bytes = fixture(&payload, 1);
        fs::write(&path, &bytes).unwrap();
        assert!(matches!(
            Log::open(&path, Options::default()),
            Err(Error::Corruption { .. })
        ));
        assert_eq!(fs::read(&path).unwrap(), bytes);
    }
}

#[test]
fn second_owner_is_busy_then_clean_reopen_succeeds() {
    let temp = Temp::new();
    let path = temp.file();
    let log = read_log(&path);
    assert!(matches!(
        Log::open(&path, Options::default()),
        Err(Error::Busy)
    ));
    log.append(String::from("one")).unwrap();
    drop(log);
    let log = read_log(&path);
    assert_eq!(log.append(String::from("two")).unwrap().seq(), 2);
}

#[test]
fn pages_obey_count_bytes_ranges_and_report_first_oversized_entry() {
    let temp = Temp::new();
    let options = Options {
        max_query_bytes: 16,
        ..Options::default()
    };
    let log = Log::open(temp.file(), options).unwrap();
    for _ in 0..3 {
        log.append(Properties::new()).unwrap();
    }
    log.append(String::from("bigger than the page")).unwrap();
    let page = log
        .query(Query {
            after: None,
            through: Some(3),
            limit: 10,
        })
        .unwrap();
    assert_eq!(
        page.entries
            .iter()
            .map(|e| e.record.seq())
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(page.next_after, Some(2));
    let page = log
        .query(Query {
            after: page.next_after,
            through: Some(3),
            limit: 10,
        })
        .unwrap();
    assert_eq!(page.entries[0].record.seq(), 3);
    assert_eq!(page.next_after, None);
    assert!(matches!(
        log.query(Query {
            after: Some(3),
            through: None,
            limit: 10
        }),
        Err(Error::ResourceLimit {
            seq: 4,
            max_bytes: 16,
            ..
        })
    ));
    let page = log
        .query(Query {
            after: Some(2),
            through: Some(3),
            limit: 1,
        })
        .unwrap();
    assert_eq!(page.next_after, Some(3));
    assert!(
        log.query(Query {
            after: page.next_after,
            through: Some(3),
            limit: 1
        })
        .unwrap()
        .entries
        .is_empty()
    );
    assert!(matches!(
        log.query(Query {
            limit: 0,
            ..Query::default()
        }),
        Err(Error::InvalidQuery(_))
    ));
}

#[test]
fn concurrent_appends_and_independent_read_cursors_preserve_all_entries() {
    let temp = Temp::new();
    let path = temp.file();
    let log = Arc::new(read_log(&path));
    let barrier = Arc::new(Barrier::new(4));
    let mut threads = Vec::new();
    for worker in 0..3 {
        let log = Arc::clone(&log);
        let barrier = Arc::clone(&barrier);
        threads.push(std::thread::spawn(move || {
            barrier.wait();
            let mut assigned = Vec::new();
            for index in 0..40 {
                let text = format!("worker-{worker}-{index}");
                let record = log.append(text.clone()).unwrap();
                assert_eq!(
                    log.read(record.seq()).unwrap().unwrap().properties,
                    vec![("content".into(), Value::Text(text))]
                );
                assigned.push(record.seq());
            }
            assigned
        }));
    }
    barrier.wait();
    for _ in 0..40 {
        let entries = log.query(Query::default()).unwrap().entries;
        assert!(
            entries
                .windows(2)
                .all(|pair| pair[1].record.seq() == pair[0].record.seq() + 1)
        );
    }
    let mut assigned: Vec<_> = threads
        .into_iter()
        .flat_map(|t| t.join().unwrap())
        .collect();
    assigned.sort_unstable();
    assert_eq!(assigned, (1..=120).collect::<Vec<_>>());
    drop(log);
    let log = read_log(&path);
    assert_eq!(
        log.query(Query {
            limit: 120,
            ..Query::default()
        })
        .unwrap()
        .entries
        .len(),
        120
    );
}

#[test]
fn explicit_sync_mode_writes_a_readable_record() {
    let temp = Temp::new();
    let log = Log::open(
        temp.file(),
        Options {
            completion: deleuze::Completion::Synced,
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(
        log.append(String::from("synced file operation"))
            .unwrap()
            .seq(),
        1
    );
    assert!(log.read(1).unwrap().is_some());
}
