#!/bin/bash

echo "🧪 SpiraChain Spiral Generation Benchmark"
echo "=========================================="
echo ""

# Detect hardware
echo "📊 Hardware Detection:"
echo "   CPU: $(grep -m 1 'model name' /proc/cpuinfo | cut -d: -f2 | xargs)"
echo "   Cores: $(nproc)"
echo "   RAM: $(free -h | grep Mem | awk '{print $2}')"
echo "   OS: $(uname -s) $(uname -m)"
echo ""

# Check if SpiraPi is available
SPIRAPI_AVAILABLE=false
if [ -d "crates/spirapi" ]; then
    echo "✅ SpiraPi found"
    SPIRAPI_AVAILABLE=true
else
    echo "⚠️  SpiraPi not found (AI disabled)"
fi
echo ""

# Benchmark 1: ID Generation (without AI)
echo "📈 Benchmark 1: ID Generation (Pure Math)"
echo "----------------------------------------"

cat > /tmp/benchmark_ids.py << 'EOF'
import time
import sys
import os

sys.path.insert(0, os.path.join(os.getcwd(), 'crates/spirapi/src'))

try:
    from math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
    
    engine = PiDIndexationEngine(
        precision=PrecisionLevel.MEDIUM,
        algorithm=PiAlgorithm.CHUDNOVSKY
    )
    
    # Warm-up
    print("   Warming up...")
    engine.generate_unique_identifier(length=20)
    
    # Benchmark small batch
    print("   Testing 100 IDs...")
    start = time.perf_counter()
    results = engine.generate_batch_identifiers(count=100, length=20, include_spiral=True)
    elapsed = time.perf_counter() - start
    rate_100 = 100 / elapsed
    print(f"   ✓ 100 IDs: {elapsed:.4f}s ({rate_100:.0f} IDs/sec)")
    
    # Benchmark large batch
    print("   Testing 1,000 IDs...")
    start = time.perf_counter()
    results = engine.generate_batch_identifiers(count=1000, length=20, include_spiral=True)
    elapsed = time.perf_counter() - start
    rate_1000 = 1000 / elapsed
    print(f"   ✓ 1,000 IDs: {elapsed:.4f}s ({rate_1000:.0f} IDs/sec)")
    
    # Benchmark ultra batch
    print("   Testing 10,000 IDs...")
    start = time.perf_counter()
    results = engine.generate_batch_identifiers(count=10000, length=20, include_spiral=True)
    elapsed = time.perf_counter() - start
    rate_10000 = 10000 / elapsed
    print(f"   ✓ 10,000 IDs: {elapsed:.4f}s ({rate_10000:.0f} IDs/sec)")
    
    print("")
    print(f"🎉 PEAK PERFORMANCE: {rate_10000:.0f} IDs/sec")
    
except Exception as e:
    print(f"   ❌ Error: {e}")
    print("   SpiraPi not properly configured")
EOF

if [ "$SPIRAPI_AVAILABLE" = true ]; then
    python3 /tmp/benchmark_ids.py
else
    echo "   ⚠️  Skipped (SpiraPi not available)"
fi
echo ""

# Benchmark 2: Spiral Complexity Calculation
echo "📈 Benchmark 2: Spiral Complexity (CPU Only)"
echo "--------------------------------------------"

cat > /tmp/benchmark_spirals.rs << 'EOF'
use std::time::Instant;

fn calculate_spiral_complexity(x: f64, y: f64, z: f64) -> f64 {
    let r = (x * x + y * y).sqrt();
    let theta = y.atan2(x);
    let spiral_factor = r * theta.abs();
    let geometric_complexity = (spiral_factor * 100.0).min(300.0);
    
    // Add z-axis contribution
    let z_factor = z.abs() * 10.0;
    
    (geometric_complexity + z_factor).max(50.0).min(250.0)
}

fn main() {
    println!("   Warming up...");
    
    // Warm-up
    for i in 0..1000 {
        let x = (i as f64) * 0.1;
        let y = (i as f64) * 0.2;
        let z = (i as f64) * 0.05;
        calculate_spiral_complexity(x, y, z);
    }
    
    // Benchmark
    let counts = [1000, 10000, 100000];
    
    for count in counts {
        println!("   Testing {} spirals...", count);
        let start = Instant::now();
        
        for i in 0..count {
            let x = (i as f64) * 0.1;
            let y = (i as f64) * 0.2;
            let z = (i as f64) * 0.05;
            calculate_spiral_complexity(x, y, z);
        }
        
        let elapsed = start.elapsed().as_secs_f64();
        let rate = count as f64 / elapsed;
        
        println!("   ✓ {} spirals: {:.4}s ({:.0} spirals/sec)", count, elapsed, rate);
    }
}
EOF

# Compile and run Rust benchmark
rustc -O /tmp/benchmark_spirals.rs -o /tmp/benchmark_spirals 2>/dev/null
if [ $? -eq 0 ]; then
    /tmp/benchmark_spirals
else
    echo "   ⚠️  Rust compiler not available"
fi
echo ""

# Benchmark 3: With AI Semantic Analysis
if [ "$SPIRAPI_AVAILABLE" = true ]; then
    echo "📈 Benchmark 3: Spiral Generation WITH AI"
    echo "-----------------------------------------"
    
    cat > /tmp/benchmark_with_ai.py << 'EOF'
import time
import sys
import os

sys.path.insert(0, os.path.join(os.getcwd(), 'crates/spirapi/src'))

try:
    from math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
    
    # Check if sentence-transformers is available
    try:
        from sentence_transformers import SentenceTransformer
        model = SentenceTransformer('sentence-transformers/all-MiniLM-L6-v2')
        AI_AVAILABLE = True
        print("   ✅ AI Model loaded")
    except:
        AI_AVAILABLE = False
        print("   ⚠️  AI Model not available")
    
    if AI_AVAILABLE:
        # Simulate semantic analysis
        test_transactions = [
            "Payment for coffee",
            "Salary transfer",
            "Rent payment",
            "Grocery shopping",
            "Investment deposit"
        ]
        
        print("   Testing AI semantic analysis...")
        start = time.perf_counter()
        
        for i in range(10):  # 10 blocks with 5 transactions each
            # Generate spiral coordinates
            engine = PiDIndexationEngine(
                precision=PrecisionLevel.MEDIUM,
                algorithm=PiAlgorithm.CHUDNOVSKY
            )
            spiral_id = engine.generate_unique_identifier(length=20)
            
            # Analyze transactions (AI part)
            embeddings = model.encode(test_transactions)
            coherence = float(embeddings.mean())
            
        elapsed = time.perf_counter() - start
        rate = 10 / elapsed
        
        print(f"   ✓ 10 blocks (with AI): {elapsed:.4f}s ({rate:.1f} blocks/sec)")
        print(f"   ✓ Per block: {elapsed/10:.4f}s")
        print("")
        print(f"🎉 WITH AI: {rate:.1f} spirals/sec")
        print(f"   Note: Slower but +50% rewards!")
        
except Exception as e:
    print(f"   ❌ Error: {e}")
EOF

    python3 /tmp/benchmark_with_ai.py
else
    echo "   ⚠️  Skipped (SpiraPi not available)"
fi
echo ""

# Summary
echo "📊 BENCHMARK SUMMARY"
echo "===================="
echo ""
echo "Hardware Capability:"
echo "   • Pure Math (no AI): ~10,000-100,000 spirals/sec"
echo "   • With AI Analysis:  ~20-100 spirals/sec"
echo ""
echo "Validation Requirement:"
echo "   • 1 spiral every 30 seconds"
echo "   • Even with AI, you have 600-3000x more power than needed!"
echo ""
echo "Recommendation:"
echo "   • Use AI for maximum rewards (+50%)"
echo "   • Performance is more than sufficient"
echo ""

# Cleanup
rm -f /tmp/benchmark_*.py /tmp/benchmark_*.rs /tmp/benchmark_spirals

echo "✅ Benchmark Complete!"

