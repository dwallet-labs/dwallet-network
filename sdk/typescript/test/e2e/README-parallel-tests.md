# Parallel Performance Tests

This test suite provides comprehensive parallel performance testing for dWallet MPC operations including DKG, Presign, Sign, and full flow operations.

## Overview

The parallel performance tests are designed to:
1. **Run DKG X Times** - Execute multiple DKG operations in parallel
2. **Run DKG once and Presign X times** - Create one dWallet and run multiple Presign operations
3. **Run DKG once, Presign once, Sign X times** - Setup once and run multiple Sign operations
4. **Run full flow (DKG + Presign + Sign) X times** - Complete end-to-end operations in parallel
5. **Performance comparison** - Compare individual operation performance

## Configuration

### Environment Variables

- `PARALLEL_COUNT` - Number of parallel operations to run (default: 8)
- `TEST_TIMEOUT` - Timeout per test in milliseconds (default: 600000 = 10 minutes)

### Examples

```bash
# Run with 4 parallel operations
PARALLEL_COUNT=4 npm test parallel-performance.test.ts

# Run with 16 parallel operations and extended timeout
PARALLEL_COUNT=16 TEST_TIMEOUT=1200000 npm test parallel-performance.test.ts

# Run with default settings (8 parallel)
npm test parallel-performance.test.ts
```

## Test Cases

### 1. Parallel DKG Test
```typescript
it('should run DKG X times in parallel')
```
- Creates X dWallets simultaneously
- Measures individual DKG operation times
- Reports success rate and performance metrics

### 2. Single DKG + Parallel Presign Test
```typescript
it('should run DKG once and Presign X times in parallel')
```
- Creates one dWallet
- Runs X Presign operations in parallel using the same dWallet
- Measures Presign performance when sharing the same dWallet

### 3. Single Setup + Parallel Sign Test
```typescript
it('should run DKG once, Presign once, and Sign X times in parallel')
```
- Creates one dWallet and one Presign
- Runs X Sign operations in parallel using the same setup
- Each Sign operation uses a unique message
- Measures Sign performance with shared setup

### 4. Parallel Full Flow Test
```typescript
it('should run full flow (DKG + Presign + Sign) X times in parallel')
```
- Runs complete DKG → Presign → Sign flow X times in parallel
- Each operation is completely independent
- Measures end-to-end performance
- Uses double timeout due to complexity

### 5. Performance Comparison Test
```typescript
it('should run comprehensive performance comparison')
```
- Runs a smaller number of each operation type
- Provides side-by-side performance comparison
- Useful for understanding relative performance characteristics

## Output Format

### Individual Operation Logging
```
[DKG] #1: SUCCESS - 2341ms
[DKG] #2: SUCCESS - 2156ms
[DKG] #3: FAILED - 0ms (Network timeout)
```

### Performance Summary
```
📊 DKG Performance Summary:
   Total Operations: 8
   Successful: 7
   Failed: 1
   Success Rate: 87.5%
   Average Duration: 2248ms
   Min Duration: 1987ms
   Max Duration: 2634ms
   Total Wall Time: 3456ms
   Throughput: 2.03 ops/sec

❌ Failed Operations:
   #3: Network timeout
```

### Comprehensive Comparison
```
📊 COMPREHENSIVE PERFORMANCE COMPARISON:
=====================================
DKG     : avg=2248ms, min=1987ms, max=2634ms
Presign : avg=1456ms, min=1234ms, max=1678ms
Sign    : avg=892ms, min=756ms, max=1023ms
```

## Performance Metrics

Each test tracks the following metrics:
- **Operation Type** - DKG, Presign, Sign, or Full Flow
- **Individual Timing** - Start time, end time, duration for each operation
- **Success Rate** - Percentage of successful operations
- **Statistical Analysis** - Average, minimum, maximum durations
- **Throughput** - Operations per second
- **Wall Clock Time** - Total time for all parallel operations
- **Error Tracking** - Details of any failed operations

## Usage Recommendations

### Development Testing
```bash
# Quick test with fewer operations
PARALLEL_COUNT=2 npm test parallel-performance.test.ts
```

### Load Testing
```bash
# Stress test with many parallel operations
PARALLEL_COUNT=20 TEST_TIMEOUT=1800000 npm test parallel-performance.test.ts
```

### CI/CD Integration
```bash
# Moderate load for continuous integration
PARALLEL_COUNT=4 TEST_TIMEOUT=900000 npm test parallel-performance.test.ts
```

## Interpreting Results

### Success Rate
- **>95%**: Excellent - System handling load well
- **85-95%**: Good - Some occasional failures expected
- **70-85%**: Concerning - May indicate resource constraints
- **<70%**: Poor - System struggling with load

### Performance Trends
- **DKG**: Typically the slowest operation (2-5 seconds)
- **Presign**: Medium duration (1-3 seconds)
- **Sign**: Fastest operation (0.5-1.5 seconds)
- **Full Flow**: Sum of all three plus overhead

### Throughput Analysis
- Higher throughput indicates better parallel processing capability
- Compare wall clock time vs sum of individual operations
- Lower ratios indicate better parallelization efficiency

## Troubleshooting

### Common Issues

1. **Network Timeouts**
   - Increase `TEST_TIMEOUT`
   - Reduce `PARALLEL_COUNT`
   - Check network connectivity

2. **Resource Exhaustion**
   - Reduce `PARALLEL_COUNT`
   - Monitor system resources (CPU, memory, network)
   - Check for rate limiting

3. **Faucet Limitations**
   - Tests request SUI from faucet
   - May hit rate limits with high parallel counts
   - Consider using funded accounts for load testing

### Performance Optimization

1. **Optimal Parallel Count**
   - Start with 4-8 parallel operations
   - Increase gradually while monitoring success rate
   - Find the sweet spot where throughput peaks

2. **Timeout Tuning**
   - Set timeout 2-3x expected operation time
   - Account for network variability
   - Full flow tests need longer timeouts

3. **Resource Monitoring**
   - Monitor CPU, memory, and network usage
   - Watch for bottlenecks in the system
   - Consider distributed testing for higher loads

## Integration with CI/CD

Add to your test pipeline:

```yaml
# GitHub Actions example
- name: Run Parallel Performance Tests
  run: |
    PARALLEL_COUNT=4 TEST_TIMEOUT=900000 npm test parallel-performance.test.ts
  env:
    NODE_ENV: test
```

The tests are designed to be robust and provide meaningful performance insights for dWallet MPC operations at scale. 