#!/usr/bin/env python3
import re
import os

def extract_times(filename):
    """Extract proving times from log file by looking for 'Total proof generation' lines"""
    times = []
    try:
        with open(filename, 'r') as f:
            for line in f:
                if "Total proof generation" in line and "End:" in line:
                    # Extract the time value - handle both seconds and milliseconds
                    # Examples: "37.673s" or "824.982ms"
                    match_s = re.search(r'(\d+\.\d+)s', line)
                    match_ms = re.search(r'(\d+\.\d+)ms', line)
                    
                    if match_s:
                        times.append(float(match_s.group(1)))
                    elif match_ms:
                        # Convert milliseconds to seconds
                        times.append(float(match_ms.group(1)) / 1000.0)
    except FileNotFoundError:
        pass
    return times

def format_time(seconds):
    """Format time in seconds to readable format"""
    if seconds < 1:
        return f"{seconds*1000:.1f}ms"
    else:
        return f"{seconds:.2f}s"

# Thread counts to process
threads = [1, 2, 4, 8, 16]
base_path = "/home/zhenfei/Desktop/miden/playground/"

# Test configurations mapping
test_configs = [
    (65536, 40),   # 65K steps, 40 cols
    (65536, 80),   # 65K steps, 80 cols
    (1048576, 40), # 1M steps, 40 cols
    (1048576, 80)  # 1M steps, 80 cols
]

print("# Proving Time Comparison Table")
print()
print("| Steps | Cols | Threads | P3 (AVX512) | P3 | Winterfell |")
print("|-------|------|---------|-------------|----|-----------| ")

for config_idx, (steps, cols) in enumerate(test_configs):
    # Format steps in readable form
    steps_str = f"{steps//1000}K" if steps < 1000000 else f"{steps//1000000}M"
    
    for thread_count in threads:
        # Extract times for each configuration
        p3_avx512_times = extract_times(f"{base_path}{thread_count}_thread_p3_avx512.log")
        p3_times = extract_times(f"{base_path}{thread_count}_thread_p3.log")  
        wf_times = extract_times(f"{base_path}{thread_count}_thread_wf.log")
        
        # Get the specific test case time
        p3_avx512_time = p3_avx512_times[config_idx] if config_idx < len(p3_avx512_times) else 0
        p3_time = p3_times[config_idx] if config_idx < len(p3_times) else 0
        wf_time = wf_times[config_idx] if config_idx < len(wf_times) else 0
        
        # Format the results
        p3_avx512_str = format_time(p3_avx512_time) if p3_avx512_time > 0 else "N/A"
        p3_str = format_time(p3_time) if p3_time > 0 else "N/A"
        wf_str = format_time(wf_time) if wf_time > 0 else "N/A"
        
        print(f"| {steps_str:5} | {cols:4} | {thread_count:7} | {p3_avx512_str:11} | {p3_str:10} | {wf_str:10} |")

