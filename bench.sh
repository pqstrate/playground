#! /bin/bash

# ######################################
# # P3-Monty demos with Keccak hash function 
# ######################################
# NUM_THREADS=1 HASH_TYPE=blake3 cargo run --release --bin p3_monty_demo > 1_thread_p3_monty_blake3.log

# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=8 HASH_TYPE=keccak cargo run --release --bin p3_monty_demo > 8_thread_p3_monty_keccak_avx512.log
# # NUM_THREADS=8 HASH_TYPE=keccak cargo run --release --bin p3_monty_demo > 8_thread_p3_monty_keccak.log

# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=16 HASH_TYPE=keccak cargo run --release --bin p3_monty_demo > 16_thread_p3_monty_keccak_avx512.log
# NUM_THREADS=16 HASH_TYPE=blake3 cargo run --release --bin p3_monty_demo > 16_thread_p3_monty_blake3.log

# # # # P3-Monty demos with Poseidon2 hash function
# NUM_THREADS=1 HASH_TYPE=poseidon2 cargo run --release --bin p3_monty_demo > 1_thread_p3_monty_poseidon2.log
# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=8 HASH_TYPE=poseidon2 cargo run --release --bin p3_monty_demo > 8_thread_p3_monty_poseidon2_avx512.log
# # NUM_THREADS=8 HASH_TYPE=poseidon2 cargo run --release --bin p3_monty_demo > 8_thread_p3_monty_poseidon2.log

# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=16 HASH_TYPE=poseidon2 cargo run --release --bin p3_monty_demo > 16_thread_p3_monty_poseidon2_avx512.log
# NUM_THREADS=16 HASH_TYPE=poseidon2 cargo run --release --bin p3_monty_demo > 16_thread_p3_monty_poseidon2.log

# # ######################################
# # # P3 demos with blake3 hash function
# # ######################################
# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=1 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 1_thread_p3_blake3_avx512.log
NUM_THREADS=1 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 1_thread_p3_blake3.log

# # # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=2 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 2_thread_p3_blake3_avx512.log
# # # NUM_THREADS=2 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 2_thread_p3_blake3.log

# # # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=4 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 4_thread_p3_blake3_avx512.log
# # # NUM_THREADS=4 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 4_thread_p3_blake3.log

# # # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=8 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 8_thread_p3_blake3_avx512.log
# # # NUM_THREADS=8 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 8_thread_p3_blake3.log

# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=16 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 16_thread_p3_blake3_avx512.log
NUM_THREADS=16 HASH_TYPE=blake3 cargo run --release --bin p3_demo > 16_thread_p3_blake3.log

# # ######################################
# # # P3 demos with Keccak hash function
# # ######################################
# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=1 HASH_TYPE=keccak cargo run --release --bin p3_demo > 1_thread_p3_keccak_avx512.log
# # NUM_THREADS=1 HASH_TYPE=keccak cargo run --release --bin p3_demo > 1_thread_p3_keccak.log

# # # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=2 HASH_TYPE=keccak cargo run --release --bin p3_demo > 2_thread_p3_keccak_avx512.log
# # # NUM_THREADS=2 HASH_TYPE=keccak cargo run --release --bin p3_demo > 2_thread_p3_keccak.log

# # # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=4 HASH_TYPE=keccak cargo run --release --bin p3_demo > 4_thread_p3_keccak_avx512.log
# # # NUM_THREADS=4 HASH_TYPE=keccak cargo run --release --bin p3_demo > 4_thread_p3_keccak.log

# # # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=8 HASH_TYPE=keccak cargo run --release --bin p3_demo > 8_thread_p3_keccak_avx512.log
# # # NUM_THREADS=8 HASH_TYPE=keccak cargo run --release --bin p3_demo > 8_thread_p3_keccak.log

# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=16 HASH_TYPE=keccak cargo run --release --bin p3_demo > 16_thread_p3_keccak_avx512.log
# # NUM_THREADS=16 HASH_TYPE=keccak cargo run --release --bin p3_demo > 16_thread_p3_keccak.log

# # ######################################
# # # P3 demos with Poseidon2 hash function
# # ######################################
# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=1 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 1_thread_p3_poseidon2_avx512.log
# NUM_THREADS=1 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 1_thread_p3_poseidon2.log

# # # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=2 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 2_thread_p3_poseidon2_avx512.log
# # # NUM_THREADS=2 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 2_thread_p3_poseidon2.log

# # # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=4 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 4_thread_p3_poseidon2_avx512.log
# # # NUM_THREADS=4 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 4_thread_p3_poseidon2.log

# # # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=8 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 8_thread_p3_poseidon2_avx512.log
# # # NUM_THREADS=8 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 8_thread_p3_poseidon2.log

# # RUSTFLAGS="-C target-feature=+avx512f" NUM_THREADS=16 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 16_thread_p3_poseidon2_avx512.log
# NUM_THREADS=16 HASH_TYPE=poseidon2 cargo run --release --bin p3_demo > 16_thread_p3_poseidon2.log

######################################
# WF demos with Blake3_256 hash function
######################################
# NUM_THREADS=1 HASH_TYPE=blake256 cargo run --release --bin wf_demo > 1_thread_wf_blake256.log
# # NUM_THREADS=2 HASH_TYPE=blake256 cargo run --release --bin wf_demo > 2_thread_wf_blake256.log
# # NUM_THREADS=4 HASH_TYPE=blake256 cargo run --release --bin wf_demo > 4_thread_wf_blake256.log
# # NUM_THREADS=8 HASH_TYPE=blake256 cargo run --release --bin wf_demo > 8_thread_wf_blake256.log
# NUM_THREADS=16 HASH_TYPE=blake256 cargo run --release --bin wf_demo > 16_thread_wf_blake256.log

# ######################################
# # WF demos with Poseidon2 hash function (slow)
# ######################################
# NUM_THREADS=1 HASH_TYPE=poseidon2 cargo run --release --bin wf_demo > 1_thread_wf_poseidon2.log
# # NUM_THREADS=2 HASH_TYPE=poseidon2 cargo run --release --bin wf_demo > 2_thread_wf_poseidon2.log
# # NUM_THREADS=4 HASH_TYPE=poseidon2 cargo run --release --bin wf_demo > 4_thread_wf_poseidon2.log
# # NUM_THREADS=8 HASH_TYPE=poseidon2 cargo run --release --bin wf_demo > 8_thread_wf_poseidon2.log
# NUM_THREADS=16 HASH_TYPE=poseidon2 cargo run --release --bin wf_demo > 16_thread_wf_poseidon2.log