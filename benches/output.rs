use core::hint::black_box;
use std::io::BufWriter;
use std::io::Write as _;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::DistString as _;
use rand::distributions::Standard;

// TODO: Test `println!` and use actually stdout

fn string_push_str_with_format(ss: &[String]) {
    let mut buf = String::new();

    for s in ss {
        buf.push_str(&format!("{s}\n"));
    }
}

fn string_push_str_without_format(ss: &[String]) {
    let mut buf = String::new();

    for s in ss {
        buf.push_str(s);
        buf.push('\n');
    }
}

#[allow(clippy::unwrap_used)]
fn buf_writer_writeln(ss: &[String]) {
    let buf = vec![];
    let mut output = BufWriter::new(buf);

    for s in ss {
        writeln!(output, "{s}").unwrap();
    }
}

#[allow(clippy::unwrap_used)]
#[allow(clippy::write_with_newline)]
fn buf_writer_write(ss: &[String]) {
    let buf = vec![];
    let mut output = BufWriter::new(buf);

    for s in ss {
        write!(output, "{s}\n").unwrap();
    }
}

#[allow(clippy::unwrap_used)]
#[allow(clippy::unused_io_amount)]
fn buf_writer_write_method_with_format(ss: &[String]) {
    let buf = vec![];
    let mut output = BufWriter::new(buf);

    for s in ss {
        output.write(format!("{s}\n").as_bytes()).unwrap();
    }
}

#[allow(clippy::unwrap_used)]
#[allow(clippy::unused_io_amount)]
fn buf_writer_write_method_without_format(ss: &[String]) {
    let buf = vec![];
    let mut output = BufWriter::new(buf);

    for s in ss {
        output.write(s.as_bytes()).unwrap();
        output.write(b"\n").unwrap();
    }
}

#[allow(clippy::unwrap_used)]
fn buf_writer_write_all_push_str(ss: &[String]) {
    let buf = vec![];
    let mut output = BufWriter::new(buf);

    for chunk in ss.chunks(100) {
        let mut chunk_buf = String::new();

        for s in chunk {
            chunk_buf.push_str(&format!("{s}\n"));
        }

        output.write_all(chunk_buf.as_bytes()).unwrap();
    }
}

#[allow(clippy::unwrap_used)]
fn buf_writer_write_all_append_with_format(ss: &[String]) {
    let buf = vec![];
    let mut output = BufWriter::new(buf);

    for chunk in ss.chunks(100) {
        let mut chunk_buf = String::new();

        for s in chunk {
            chunk_buf += &format!("{s}\n");
        }

        output.write_all(chunk_buf.as_bytes()).unwrap();
    }
}

#[allow(clippy::unwrap_used)]
fn buf_writer_write_all_append_without_format(ss: &[String]) {
    let buf = vec![];
    let mut output = BufWriter::new(buf);

    for chunk in ss.chunks(100) {
        let mut chunk_buf = String::new();

        for s in chunk {
            chunk_buf += s;
            chunk_buf += "\n";
        }

        output.write_all(chunk_buf.as_bytes()).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("output");

    let mut ss = vec![];
    for _ in 0..50000 {
        let s = Standard.sample_string(&mut rand::thread_rng(), 128);
        ss.push(s);
    }

    group.bench_function("string_push_str_with_format", |b| {
        b.iter(|| string_push_str_with_format(black_box(&ss.clone())));
    });

    group.bench_function("string_push_str_without_format", |b| {
        b.iter(|| string_push_str_without_format(black_box(&ss.clone())));
    });

    group.bench_function("buf_writer_writeln", |b| {
        b.iter(|| buf_writer_writeln(black_box(&ss.clone())));
    });

    group.bench_function("buf_writer_write", |b| {
        b.iter(|| buf_writer_write(black_box(&ss.clone())));
    });

    group.bench_function("buf_writer_write_method_with_format", |b| {
        b.iter(|| buf_writer_write_method_with_format(black_box(&ss.clone())));
    });

    group.bench_function("buf_writer_write_method_without_format", |b| {
        b.iter(|| buf_writer_write_method_without_format(black_box(&ss.clone())));
    });

    group.bench_function("buf_writer_write_all_push_str", |b| {
        b.iter(|| buf_writer_write_all_push_str(black_box(&ss.clone())));
    });

    group.bench_function("buf_writer_write_all_append_with_format", |b| {
        b.iter(|| buf_writer_write_all_append_with_format(black_box(&ss.clone())));
    });

    group.bench_function("buf_writer_write_all_append_without_format", |b| {
        b.iter(|| buf_writer_write_all_append_without_format(black_box(&ss.clone())));
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
