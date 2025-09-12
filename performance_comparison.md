# STARK Prover Performance Comparison: P3 vs Winterfell

## Test Configuration
- **Constraint**: `x₁⁸ + x₂ + ... + x_{n-1} = xₙ`  
- **Transition**: `next_x₁ = current_xₙ`
- **Field**: Goldilocks (P3) vs BaseElement (Winterfell)
- **Single Thread**: `RAYON_NUM_THREADS=1`

## Performance Comparison Table

### P3 (Standard) vs Winterfell vs P3 (AVX512f)

| Steps | Cols | Threads | P3 (AVX512) | P3 | Winterfell |
|-------|------|---------|-------------|----|-----------| 
| 65K   |   40 |       1 | 1.17s       | 1.51s      | 2.52s      |
| 65K   |   40 |       2 | 625.3ms     | 807.0ms    | 1.29s      |
| 65K   |   40 |       4 | 353.1ms     | 433.4ms    | 708.8ms    |
| 65K   |   40 |       8 | 211.2ms     | 257.7ms    | 433.0ms    |
| 65K   |   40 |      16 | 180.5ms     | 204.6ms    | 346.3ms    |
| | | | | | |
| 65K   |   80 |       1 | 1.53s       | 2.08s      | 4.32s      |
| 65K   |   80 |       2 | 825.0ms     | 1.08s      | 2.24s      |
| 65K   |   80 |       4 | 450.1ms     | 584.4ms    | 1.21s      |
| 65K   |   80 |       8 | 265.7ms     | 338.3ms    | 724.9ms    |
| 65K   |   80 |      16 | 222.1ms     | 244.5ms    | 570.7ms    |
| | | | | | |
| 1M    |   40 |       1 | 21.08s      | 26.44s     | 46.32s     |
| 1M    |   40 |       2 | 11.28s      | 13.98s     | 24.34s     |
| 1M    |   40 |       4 | 6.37s       | 7.70s      | 13.53s     |
| 1M    |   40 |       8 | 3.82s       | 4.44s      | 7.69s      |
| 1M    |   40 |      16 | 2.72s       | 3.01s      | 5.25s      |
| | | | | | |
| 1M    |   80 |       1 | 29.95s      | 37.67s     | 81.07s     |
| 1M    |   80 |       2 | 15.81s      | 19.48s     | 41.98s     |
| 1M    |   80 |       4 | 8.74s       | 10.59s     | 23.51s     |
| 1M    |   80 |       8 | 5.15s       | 6.08s      | 13.27s     |
| 1M    |   80 |      16 | 3.58s       | 3.98s      | 9.00s      |
