#!/usr/bin/env python3
"""
SpiraPi Mathematical Engine Demo
Demonstrates the π-based mathematical engine capabilities
"""

import sys
import os
import time
from pathlib import Path

# Add project paths
current_dir = os.path.dirname(os.path.abspath(__file__))
project_root = os.path.dirname(current_dir)
sys.path.insert(0, project_root)
sys.path.insert(0, os.path.join(project_root, 'src'))

def print_banner():
    """Print mathematical engine banner"""
    print("""
    ╔══════════════════════════════════════════════════════════════╗
    ║                   🧮 MATHEMATICAL ENGINE                     ║
    ║                                                              ║
    ║              π-Based Sequence Generation                     ║
    ║              High-Precision Calculations                     ║
    ║              Advanced Spiral Mathematics                     ║
    ╚══════════════════════════════════════════════════════════════╝
    """)

def demo_pi_calculations():
    """Demonstrate π calculations with different algorithms"""
    print("\n🔢 π Calculation Demonstrations")
    print("="*50)
    
    try:
        from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
        
        # Test different precision levels
        precision_levels = [PrecisionLevel.LOW, PrecisionLevel.MEDIUM, PrecisionLevel.HIGH]
        algorithms = [PiAlgorithm.CHUDNOVSKY, PiAlgorithm.MACHIN, PiAlgorithm.RAMANUJAN]
        
        for precision in precision_levels:
            for algorithm in algorithms:
                print(f"\n🧮 Testing {precision.name} precision with {algorithm.name} algorithm:")
                
                start_time = time.time()
                engine = PiDIndexationEngine(precision=precision, algorithm=algorithm)
                
                # Generate a few sequences
                sequences = []
                for i in range(5):
                    seq_data = engine.generate_unique_identifier(length=10 + i*5)
                    sequences.append(seq_data['identifier'])
                
                end_time = time.time()
                generation_time = end_time - start_time
                
                print(f"  ✅ Generated {len(sequences)} sequences in {generation_time:.4f}s")
                print(f"  📊 First sequence: {sequences[0][:20]}...")
                print(f"  🎯 Precision: {precision.value} digits")
                
    except Exception as e:
        print(f"❌ Error in π calculations: {e}")

def demo_sequence_generation():
    """Demonstrate unique sequence generation"""
    print("\n🎲 Unique Sequence Generation")
    print("="*50)
    
    try:
        from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
        
        engine = PiDIndexationEngine(precision=PrecisionLevel.HIGH, algorithm=PiAlgorithm.CHUDNOVSKY)
        
        # Generate multiple sequences and check uniqueness
        sequences = []
        start_time = time.time()
        
        for i in range(100):
            seq_data = engine.generate_unique_identifier(length=20)
            sequences.append(seq_data['identifier'])
        
        end_time = time.time()
        generation_time = end_time - start_time
        
        # Check uniqueness
        unique_sequences = set(sequences)
        uniqueness_rate = len(unique_sequences) / len(sequences) * 100
        
        print(f"✅ Generated {len(sequences)} sequences in {generation_time:.4f}s")
        print(f"🎯 Uniqueness rate: {uniqueness_rate:.2f}%")
        print(f"📊 Average generation time: {generation_time/len(sequences)*1000:.2f}ms per sequence")
        
        # Show some examples
        print(f"\n📝 Sample sequences:")
        for i, seq in enumerate(sequences[:5]):
            print(f"  {i+1}. {seq[:30]}...")
            
    except Exception as e:
        print(f"❌ Error in sequence generation: {e}")

def demo_spiral_calculations():
    """Demonstrate spiral mathematical calculations"""
    print("\n🌀 Spiral Mathematical Calculations")
    print("="*50)
    
    try:
        from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
        from scripts.interrupt_handler import graceful_shutdown
        
        def draw_spiral_ascii(points, spiral_type, width=200, height=100):
            """Draw ASCII representation of spiral points"""
            if not points:
                return
                
            # Create ASCII grid
            grid = [[' ' for _ in range(width)] for _ in range(height)]
            
            # Find coordinate bounds
            x_coords = [p[0] for p in points]
            y_coords = [p[1] for p in points]
            x_min, x_max = min(x_coords), max(x_coords)
            y_min, y_max = min(y_coords), max(y_coords)
            
            # Scale points to fit grid
            for x, y in points:
                # Map coordinates to grid positions
                grid_x = int((x - x_min) / (x_max - x_min + 1e-10) * (width - 1))
                grid_y = int((y - y_min) / (y_max - y_min + 1e-10) * (height - 1))
                
                # Ensure bounds
                grid_x = max(0, min(width - 1, grid_x))
                grid_y = max(0, min(height - 1, grid_y))
                
                # Convert to grid coordinates (flip Y axis)
                grid_y = height - 1 - grid_y
                
                # Place character based on spiral type (using thin characters)
                if spiral_type == "fibonacci":
                    grid[grid_y][grid_x] = '●'  # Thin red dot for Fibonacci
                elif spiral_type == "logarithmic":
                    grid[grid_y][grid_x] = '○'  # Thin blue circle for logarithmic
                elif spiral_type == "exponential":
                    grid[grid_y][grid_x] = '◆'  # Diamond for exponential
                elif spiral_type == "hyperbolic":
                    grid[grid_y][grid_x] = '▲'  # Triangle for hyperbolic
                elif spiral_type == "lituus":
                    grid[grid_y][grid_x] = '♦'  # Diamond for lituus
                elif spiral_type == "custom":
                    grid[grid_y][grid_x] = '★'  # Star for custom
                else:
                    grid[grid_y][grid_x] = '·'  # Thin dot for others
            
            # Draw the grid (clean, no borders)
            print(f"\n🎨 ASCII Visualization of {spiral_type.upper()} Spiral:")
            
            for row in grid:
                print(''.join(row))
            
            print(f"\n📊 Points: {len(points)}, Scale: X({x_min:.2f} to {x_max:.2f}), Y({y_min:.2f} to {y_max:.2f})")
        
        with graceful_shutdown("Mathematical Engine Demo") as handler:
            # Use the REAL high-performance engine with MEDIUM precision for speed
            print("🔧 Initializing REAL PiDIndexationEngine...")
            print("   - 32 Threads + 16 Processes + Massive Cache")
            print("   - Precision: MEDIUM (1000 digits) for optimal speed")
            
            engine = PiDIndexationEngine(
                precision=PrecisionLevel.MEDIUM,  # 1000 digits instead of 10,000 for speed
                algorithm=PiAlgorithm.CHUDNOVSKY,
                enable_caching=True,
                enable_persistence=True
            )
            
            print("✅ High-performance engine initialized!")
            
            # Test different spiral calculations (only supported types)
            spiral_types = ["archimedean", "fibonacci", "logarithmic", "hyperbolic", "lituus", "exponential", "custom"]  # Removed unsupported types
            
            for spiral_type in spiral_types:
                # Check for shutdown request
                if handler.is_shutdown_requested():
                    print("🛑 Shutdown requested, stopping spiral calculations")
                    break
                    
                print(f"\n🔄 Testing {spiral_type.upper()} spiral:")
                
                # Check if spiral type is supported
                supported_types = ["archimedean", "fibonacci", "logarithmic", "hyperbolic", "lituus", "exponential", "custom"]
                if spiral_type not in supported_types:
                    print(f"  ⚠️ Spiral type '{spiral_type}' not supported, skipping...")
                    continue
                
                start_time = time.time()
                
                # Generate spiral coordinates
                if hasattr(engine, 'spiral_calculator'):
                    # Generate spiral points using the available method
                    spiral = []
                    # All spirals to 500 points for optimal visualization
                    max_points = 500
                    
                    # ⚠️ PERFORMANCE WARNING
                    if spiral_type == "fibonacci":
                        print(f"  ⚠️  WARNING: Fibonacci spiral with {max_points} points may take 8-15 minutes!")
                        print(f"     💡 Consider using Ctrl+C to stop if too slow")
                    elif spiral_type == "logarithmic":
                        print(f"  ⚠️  WARNING: Logarithmic spiral with {max_points} points may take 5-10 minutes!")
                    elif spiral_type == "hyperbolic":
                        print(f"  ⚠️  WARNING: Hyperbolic spiral with {max_points} points may take 2-5 minutes!")
                    elif spiral_type == "lituus":
                        print(f"  ⚠️  WARNING: Lituus spiral with {max_points} points may take 3-6 minutes!")
                    elif spiral_type == "exponential":
                        print(f"  ⚠️  WARNING: Exponential spiral with {max_points} points may take 4-8 minutes!")
                    elif spiral_type == "custom":
                        print(f"  ⚠️  WARNING: Custom spiral with {max_points} points may take 2-4 minutes!")
                    else:
                        print(f"  ⚡ Archimedean spiral: {max_points} points (should be fast)")
                    
                    for i in range(max_points):
                        # Check for shutdown request during loop
                        if handler.is_shutdown_requested():
                            print("🛑 Shutdown requested, stopping point generation")
                            break
                            
                        try:
                            theta = i * 0.01  # Smaller angle increment for 1000 points
                            
                            # Special handling for Fibonacci spiral
                            if spiral_type == "fibonacci":
                                theta = i * 0.02  # Smaller increment to prevent explosion
                                
                            point = engine.spiral_calculator.calculate_spiral_point(
                                theta=theta,
                                spiral_type=spiral_type
                            )
                            
                            # More aggressive value limiting for problematic spirals
                            if spiral_type == "fibonacci":
                                x = max(min(float(point[0]), 50), -50)   # Very tight limits
                                y = max(min(float(point[1]), 50), -50)
                            else:
                                x = max(min(float(point[0]), 1000), -1000)
                                y = max(min(float(point[1]), 1000), -1000)
                            
                            spiral.append((x, y))
                            
                            # Progress indicator for long calculations
                            if i % 25 == 0:
                                print(f"    📍 Generated {i+1}/{max_points} points...")
                                
                        except Exception as e:
                            print(f"    ⚠️ Skipping point {i}: {e}")
                            continue
                    
                    if spiral:  # Only show results if we have points
                        end_time = time.time()
                        calc_time = end_time - start_time
                        
                        print(f"  ✅ Generated {len(spiral)} spiral points in {calc_time:.4f}s")
                        print(f"  📍 First point: ({spiral[0][0]:.4f}, {spiral[0][1]:.4f})")
                        print(f"  📍 Last point: ({spiral[-1][0]:.4f}, {spiral[-1][1]:.4f})")
                        
                        # 🎨 DRAW THE COOL ASCII VISUALIZATION!
                        draw_spiral_ascii(spiral, spiral_type)
                        
                    else:
                        print(f"  ❌ No spiral points generated for {spiral_type}")
                else:
                    print(f"  ⚠️ Spiral calculator not available in this version")
                    
    except Exception as e:
        print(f"❌ Error in spiral calculations: {e}")

def demo_performance_metrics():
    """Demonstrate performance metrics"""
    print("\n⚡ Performance Metrics")
    print("="*50)
    
    try:
        from src.math_engine.pi_sequences import PiDIndexationEngine, PrecisionLevel, PiAlgorithm
        
        # Test performance with different configurations
        configs = [
            (PrecisionLevel.LOW, PiAlgorithm.CHUDNOVSKY, "Fast"),
            (PrecisionLevel.MEDIUM, PiAlgorithm.CHUDNOVSKY, "Balanced"),
            (PrecisionLevel.HIGH, PiAlgorithm.CHUDNOVSKY, "High Precision")
        ]
        
        results = []
        
        for precision, algorithm, name in configs:
            print(f"\n🚀 Testing {name} configuration:")
            
            start_time = time.time()
            engine = PiDIndexationEngine(precision=precision, algorithm=algorithm)
            
            # Generate sequences
            sequences = []
            for i in range(50):
                seq_data = engine.generate_unique_identifier(length=20)
                sequences.append(seq_data['identifier'])
            
            end_time = time.time()
            total_time = end_time - start_time
            avg_time = total_time / len(sequences) * 1000  # ms
            
            results.append({
                'name': name,
                'total_time': total_time,
                'avg_time': avg_time,
                'sequences': len(sequences)
            })
            
            print(f"  ✅ Generated {len(sequences)} sequences")
            print(f"  ⏱️ Total time: {total_time:.4f}s")
            print(f"  📊 Average time: {avg_time:.2f}ms per sequence")
        
        # Performance comparison
        print(f"\n📈 Performance Comparison:")
        print(f"{'Configuration':<15} {'Total Time':<12} {'Avg Time':<12} {'Sequences':<10}")
        print("-" * 55)
        for result in results:
            print(f"{result['name']:<15} {result['total_time']:<12.4f} {result['avg_time']:<12.2f} {result['sequences']:<10}")
            
    except Exception as e:
        print(f"❌ Error in performance metrics: {e}")

def main():
    """Main mathematical engine demo"""
    print_banner()
    
    print("🧮 Mathematical Engine Demo - Choose a demonstration:")
    print("1. π Calculations with different algorithms")
    print("2. Unique sequence generation")
    print("3. Spiral mathematical calculations")
    print("4. Performance metrics")
    print("5. Run all demonstrations")
    print("0. Exit")
    
    while True:
        choice = input("\nEnter your choice (0-5): ").strip()
        
        if choice == "0":
            print("👋 Exiting Mathematical Engine Demo")
            break
        elif choice == "1":
            demo_pi_calculations()
        elif choice == "2":
            demo_sequence_generation()
        elif choice == "3":
            demo_spiral_calculations()
        elif choice == "4":
            demo_performance_metrics()
        elif choice == "5":
            print("\n🚀 Running all mathematical demonstrations...")
            demo_pi_calculations()
            demo_sequence_generation()
            demo_spiral_calculations()
            demo_performance_metrics()
            print("\n✅ All mathematical demonstrations completed!")
        else:
            print("❌ Invalid choice. Please enter a number between 0-5.")
        
        if choice != "0":
            input("\nPress Enter to continue...")

if __name__ == "__main__":
    main()
