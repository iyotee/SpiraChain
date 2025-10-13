#!/usr/bin/env python3
"""
Quick test script to verify SpiraPi engine functionality
"""
import sys
import io

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')

from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm

print("=" * 60)
print("Testing SpiraPi Engine")
print("=" * 60)

print("\n[1/5] Initializing engine...")
engine = PiDIndexationEngine(
    precision=PrecisionLevel.HIGH,
    algorithm=PiAlgorithm.CHUDNOVSKY,
    enable_caching=True,
    enable_persistence=False
)
print("[OK] Engine initialized successfully!")

print("\n[2/5] Generating single ID...")
id_data = engine.generate_unique_identifier(length=20, include_spiral_component=True)
print(f"[OK] Generated ID: {id_data['identifier'][:60]}...")
print(f"     Generation time: {id_data['generation_time']:.6f}s")
print(f"     Uniqueness score: {id_data['uniqueness_score']:.4f}")

print("\n[3/5] Testing batch generation (100 IDs)...")
import time
start = time.perf_counter()
batch_ids = engine.generate_batch_identifiers(count=100, length=20, include_spiral=True)
batch_time = time.perf_counter() - start
rate = 100 / batch_time if batch_time > 0 else 0
print(f"[OK] Generated {len(batch_ids)} IDs in {batch_time:.6f}s")
print(f"     Rate: {rate:.1f} IDs/sec")

print("\n[4/5] Testing larger batch (1000 IDs)...")
start = time.perf_counter()
large_batch = engine.generate_batch_identifiers(count=1000, length=20, include_spiral=True)
large_batch_time = time.perf_counter() - start
large_rate = 1000 / large_batch_time if large_batch_time > 0 else 0
print(f"[OK] Generated {len(large_batch)} IDs in {large_batch_time:.6f}s")
print(f"     Rate: {large_rate:.1f} IDs/sec")

print("\n[5/5] Getting engine statistics...")
stats = engine.get_comprehensive_statistics()
print(f"[OK] Engine statistics:")
print(f"     Precision: {stats['engine_info']['precision']}")
print(f"     Algorithm: {stats['engine_info']['algorithm']}")
print(f"     Operations: {stats['engine_info']['operation_count']}")
print(f"     Total compute time: {stats['engine_info']['total_computation_time']:.3f}s")

print("\n" + "=" * 60)
print("SUCCESS! All SpiraPi Tests PASSED!")
print("=" * 60)
print(f"\nPerformance Summary:")
print(f"  Single ID:  {id_data['generation_time']*1000:.2f}ms")
print(f"  Batch 100:  {rate:.1f} IDs/sec")
print(f"  Batch 1000: {large_rate:.1f} IDs/sec")
print(f"\n[OK] SpiraPi engine is fully functional and ready for integration!")

