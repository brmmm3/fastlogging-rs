use std::{ path::PathBuf, time::Duration };

use fastlogging::Logging;
use criterion::{ criterion_group, criterion_main, Criterion };

fn benchmark_logging_file(c: &mut Criterion) {
    println!("Running benchmark for logging.file...");
    let mut group = c.benchmark_group("logging.file");
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);
    group.bench_function("Logging::file", |b| {
        b.iter(|| {
            let mut logging = Logging::new(
                None,
                None,
                None,
                Some(PathBuf::from("/tmp/fastlogging.log")),
                None,
                None,
                None,
                None
            ).unwrap();
            for _ in 1..10000 {
                logging.debug("Debug message".to_string()).unwrap();
                logging.info("Info message".to_string()).unwrap();
                logging.warning("Warning message".to_string()).unwrap();
                logging.error("Error message".to_string()).unwrap();
            }
            logging.shutdown(None)
        })
    });
    group.finish();
}

fn benchmarks(c: &mut Criterion) {
    benchmark_logging_file(c);
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
