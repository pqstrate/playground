#! /bin/bash

RUSTFLAGS="-C target-feature=+avx512f" RUST_LOG=trace NUM_THREADS=1 cargo run --release --bin p3_demo > 1_thread_p3_avx512.log
RUST_LOG=trace NUM_THREADS=1 cargo run --release --bin p3_demo > 1_thread_p3.log
RUST_LOG=trace NUM_THREADS=1 cargo run --release --bin wf_demo > 1_thread_wf.log

RUSTFLAGS="-C target-feature=+avx512f" RUST_LOG=trace NUM_THREADS=2 cargo run --release --bin p3_demo > 2_thread_p3_avx512.log
RUST_LOG=trace NUM_THREADS=2 cargo run --release --bin p3_demo > 2_thread_p3.log
RUST_LOG=trace NUM_THREADS=2 cargo run --release --bin wf_demo > 2_thread_wf.log


RUSTFLAGS="-C target-feature=+avx512f" RUST_LOG=trace NUM_THREADS=4 cargo run --release --bin p3_demo > 4_thread_p3_avx512.log
RUST_LOG=trace NUM_THREADS=4 cargo run --release --bin p3_demo > 4_thread_p3.log
RUST_LOG=trace NUM_THREADS=4 cargo run --release --bin wf_demo > 4_thread_wf.log

RUSTFLAGS="-C target-feature=+avx512f" RUST_LOG=trace NUM_THREADS=8 cargo run --release --bin p3_demo > 8_thread_p3_avx512.log
RUST_LOG=trace NUM_THREADS=8 cargo run --release --bin p3_demo > 8_thread_p3.log
RUST_LOG=trace NUM_THREADS=8 cargo run --release --bin wf_demo > 8_thread_wf.log

RUSTFLAGS="-C target-feature=+avx512f" RUST_LOG=trace NUM_THREADS=16 cargo run --release --bin p3_demo > 16_thread_p3_avx512.log
RUST_LOG=trace NUM_THREADS=16 cargo run --release --bin p3_demo > 16_thread_p3.log
RUST_LOG=trace NUM_THREADS=16 cargo run --release --bin wf_demo > 16_thread_wf.log