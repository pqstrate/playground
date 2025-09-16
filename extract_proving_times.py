#!/usr/bin/env python3
import re
import glob
import os
from collections import defaultdict

def extract_proving_times_from_file(filepath):
    """Extract the 4 proving times from a single log file"""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # First try the original format: "End:     Total proof generation"
    pattern1 = r'End:\s+Total proof generation\s+\.+(\d+(?:\.\d+)?(?:[a-z]+)?)'
    matches1 = re.findall(pattern1, content)
    
    # If that doesn't work, try the p3_monty format: "prove:prove: p3_uni_stark::prover: close time.busy="
    if len(matches1) == 0:
        pattern2 = r'run_example_[^:]*:prove:prove:\s+p3_uni_stark::prover:\s+close\s+time\.busy=(\d+(?:\.\d+)?(?:[a-z]+)?)'
        matches2 = re.findall(pattern2, content)
        matches = matches2
    else:
        matches = matches1
    
    # Also extract the benchmark parameters (steps and columns) for each test
    steps_pattern = r'Number of steps: (\d+), Columns: (\d+)'
    params = re.findall(steps_pattern, content)
    
    if len(matches) == 4 and len(params) == 4:
        return list(zip(params, matches))
    else:
        print(f"Warning: Expected 4 benchmarks in {filepath}, found {len(matches)} proving times and {len(params)} parameter sets")
        return list(zip(params[:len(matches)], matches))

def parse_filename(filename):
    """Parse filename to extract thread count and framework info"""
    basename = os.path.basename(filename)
    
    # Extract thread count
    thread_match = re.search(r'(\d+)_thread', basename)
    threads = int(thread_match.group(1)) if thread_match else 'unknown'
    
    # Determine framework
    if 'p3_keccak_avx512' in basename:
        framework = 'P3-keccak (AVX512)'
    elif 'p3_keccak' in basename:
        framework = 'P3-keccak'
    elif 'p3_poseidon2_avx512' in basename:
        framework = 'P3-poseidon(avx512)'
    elif 'p3_poseidon2' in basename:
        framework = 'p3-poseidon'
    elif 'p3_monty_keccak_avx512' in basename:
        framework = 'P3-monty-keccak (AVX512)'
    elif 'p3_monty_keccak' in basename:
        framework = 'P3-monty-keccak'
    elif 'p3_monty_poseidon2_avx512' in basename:
        framework = 'P3-monty-poseidon2(avx512)'
    elif 'p3_monty_poseidon2' in basename:
        framework = 'P3-monty-poseidon2'
    elif 'wf_blake192' in basename:
        framework = 'Winterfell-blake'
    elif 'wf_rpo' in basename:
        framework = 'wf-rpo'
    else:
        framework = 'unknown'
    
    return threads, framework

def normalize_time_to_ms(time_str):
    """Convert time string to milliseconds"""
    if time_str.endswith('ms'):
        return float(time_str[:-2])
    elif time_str.endswith('s'):
        return float(time_str[:-1]) * 1000
    elif time_str.endswith('µs'):
        return float(time_str[:-2]) / 1000
    else:
        # Assume milliseconds if no unit
        return float(time_str)

def main():
    # Find all log files
    log_files = glob.glob('*.log')
    
    # Dictionary to store results: {(steps, cols): {framework: {threads: time}}}
    results = defaultdict(lambda: defaultdict(dict))
    
    for log_file in log_files:
        threads, framework = parse_filename(log_file)
        proving_times = extract_proving_times_from_file(log_file)
        
        for (steps, cols), time_str in proving_times:
            time_ms = normalize_time_to_ms(time_str)
            results[(int(steps), int(cols))][framework][threads] = time_ms
    
    # Print results as a table
    print("Proving Times Extraction Results")
    print("=" * 80)
    
    # Get all unique frameworks and thread counts
    all_frameworks = set()
    all_threads = set()
    
    for benchmark_params, framework_data in results.items():
        for framework, thread_data in framework_data.items():
            all_frameworks.add(framework)
            all_threads.update(thread_data.keys())
    
    all_frameworks = sorted(all_frameworks)
    all_threads = sorted([t for t in all_threads if t != 'unknown'])
    
    # Print table header
    header = f"{'Steps':<10} {'Cols':<6}"
    for threads in all_threads:
        header += f" {'T=' + str(threads):<8}"
    header += f" {'Framework':<25}"
    print(header)
    print("-" * len(header))
    
    # Print data for each benchmark configuration
    for (steps, cols) in sorted(results.keys()):
        for framework in all_frameworks:
            if framework in results[(steps, cols)]:
                line = f"{steps:<10} {cols:<6}"
                thread_data = results[(steps, cols)][framework]
                
                for threads in all_threads:
                    if threads in thread_data:
                        time_ms = thread_data[threads]
                        if time_ms >= 1000:
                            time_display = f"{time_ms/1000:.2f}s"
                        else:
                            time_display = f"{time_ms:.1f}ms"
                        line += f" {time_display:<8}"
                    else:
                        line += f" {'-':<8}"
                
                line += f" {framework:<25}"
                print(line)
    
    # Also create a CSV version
    print("\n\nCSV Format:")
    print("Steps,Cols,Threads,Framework,ProvingTime_ms")
    for (steps, cols), framework_data in sorted(results.items()):
        for framework, thread_data in framework_data.items():
            for threads, time_ms in thread_data.items():
                print(f"{steps},{cols},{threads},{framework},{time_ms}")

if __name__ == "__main__":
    main()