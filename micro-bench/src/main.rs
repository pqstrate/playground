mod p3_benchmarks;

fn main() {
    println!("Hello, world!");
    p3_benchmarks::run_lde_bench();
    p3_benchmarks::run_merkle_bench();
}
