#! /bin/bash

RUST_LOG=trace NUM_THREADS=1 cargo run --release --bin p3_demo > p3_1_thread.log
RUST_LOG=trace NUM_THREADS=1 cargo run --release --bin wf_demo > wf_1_thread.log


RUST_LOG=trace NUM_THREADS=2 cargo run --release --bin p3_demo > p3_2_thread.log
RUST_LOG=trace NUM_THREADS=2 cargo run --release --bin wf_demo > wf_2_thread.log

RUST_LOG=trace NUM_THREADS=4 cargo run --release --bin p3_demo > p3_4_thread.log
RUST_LOG=trace NUM_THREADS=4 cargo run --release --bin wf_demo > wf_4_thread.log

RUST_LOG=trace NUM_THREADS=8 cargo run --release --bin p3_demo > p3_8_thread.log
RUST_LOG=trace NUM_THREADS=8 cargo run --release --bin wf_demo > wf_8_thread.log

RUST_LOG=trace NUM_THREADS=16 cargo run --release --bin p3_demo > p3_16_thread.log
RUST_LOG=trace NUM_THREADS=16 cargo run --release --bin wf_demo > wf_16_thread.log