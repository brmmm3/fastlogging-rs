use std::{path::PathBuf, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion};
use fastlogging::{FileWriterConfig, Logging, DEBUG};

fn benchmark_logging_file(c: &mut Criterion) {
    println!("Running benchmark for logging.file...");
    let mut group = c.benchmark_group("logging.file");
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);
    group.bench_function("Logging::file", |b| {
        b.iter(|| {
            let mut logging = Logging::new(
                DEBUG,
                "root",
                vec![FileWriterConfig::new(
                    DEBUG,
                    PathBuf::from("/tmp/fastlogging.log"),
                    0,
                    0,
                    None,
                    None,
                    None,
                )?
                .into()],
                None,
                None,
            )
            .unwrap();
            for _ in 1..10000 {
                logging.debug("Debug message".to_string()).unwrap();
                logging.info("Info message".to_string()).unwrap();
                logging.warning("Warning message".to_string()).unwrap();
                logging.error("Error message".to_string()).unwrap();
            }
            logging.shutdown(false)
        })
    });
    group.finish();
}

fn benchmarks(c: &mut Criterion) {
    benchmark_logging_file(c);
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
