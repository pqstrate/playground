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
        framework = 'P3-keccak (AVX512)*'  # Monty variant
    elif 'p3_monty_keccak' in basename:
        framework = 'P3-keccak*'  # Monty variant
    elif 'p3_monty_poseidon2_avx512' in basename:
        framework = 'P3-poseidon(avx512)*'  # Monty variant
    elif 'p3_monty_poseidon2' in basename:
        framework = 'p3-poseidon*'  # Monty variant
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

def format_time(time_ms):
    """Format time in a readable way"""
    if time_ms >= 1000:
        return f"{time_ms/1000:.2f}s"
    else:
        return f"{time_ms:.0f}ms"

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
    
    # Print final formatted table
    print("PROVING TIME COMPARISON TABLE")
    print("=" * 100)
    print()
    
    # Create table header based on actual requested format
    header = f"{'Steps':<10} {'Cols':<6} {'Threads':<8} {'P3-keccak (AVX512)':<18} {'P3-keccak':<12} {'Winterfell-blake':<16} {'P3-poseidon(avx512)':<18} {'p3-poseidon':<12} {'wf-rpo':<10}"
    print(header)
    print("-" * len(header))
    
    # Print data for each configuration
    for (steps, cols) in sorted(results.keys()):
        for threads in sorted([1, 2, 4, 8, 16]):
            # Create row
            row_data = {}
            for framework in results[(steps, cols)]:
                if threads in results[(steps, cols)][framework]:
                    time_ms = results[(steps, cols)][framework][threads]
                    row_data[framework] = format_time(time_ms)
                else:
                    row_data[framework] = "-"
            
            # Only print if we have at least one data point
            if any(v != "-" for v in row_data.values()):
                line = f"{steps:<10} {cols:<6} {threads:<8}"
                line += f" {row_data.get('P3-keccak (AVX512)', '-'):<18}"
                line += f" {row_data.get('P3-keccak', '-'):<12}"
                line += f" {row_data.get('Winterfell-blake', '-'):<16}"
                line += f" {row_data.get('P3-poseidon(avx512)', '-'):<18}"
                line += f" {row_data.get('p3-poseidon', '-'):<12}"
                line += f" {row_data.get('wf-rpo', '-'):<10}"
                print(line)
        print()  # Empty line between step/col groups

if __name__ == "__main__":
    main()