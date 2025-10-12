use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use spirachain_core::{Block, Transaction, Hash, Address, Amount};
use spirachain_crypto::{KeyPair, XmssKeyPair, KyberKeyPair, blake3_hash};

fn bench_block_hashing(c: &mut Criterion) {
    let mut block = Block::new(Hash::zero(), 1);
    
    for i in 0..100 {
        let tx = Transaction::new(
            Address::new([i as u8; 32]),
            Address::new([(i + 1) as u8; 32]),
            Amount::new(1000),
            Amount::new(1),
        );
        block.transactions.push(tx);
    }

    c.bench_function("block_hash_100tx", |b| {
        b.iter(|| {
            black_box(block.hash());
        })
    });
}

fn bench_transaction_signing(c: &mut Criterion) {
    let keypair = KeyPair::generate();
    let message = b"test transaction data";

    c.bench_function("transaction_sign_ed25519", |b| {
        b.iter(|| {
            black_box(keypair.sign(message));
        })
    });
}

fn bench_xmss_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("xmss");
    
    group.bench_function("keygen", |b| {
        b.iter(|| {
            black_box(XmssKeyPair::generate().unwrap());
        })
    });

    let mut keypair = XmssKeyPair::generate().unwrap();
    let message = b"test message";

    group.bench_function("sign", |b| {
        b.iter(|| {
            let mut kp = keypair.clone();
            black_box(kp.sign(message).unwrap());
        })
    });

    let signature = keypair.sign(message).unwrap();
    group.bench_function("verify", |b| {
        b.iter(|| {
            black_box(keypair.verify(message, &signature));
        })
    });

    group.finish();
}

fn bench_kyber_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("kyber");
    
    group.bench_function("keygen", |b| {
        b.iter(|| {
            black_box(KyberKeyPair::generate().unwrap());
        })
    });

    let keypair = KyberKeyPair::generate().unwrap();

    group.bench_function("encapsulate", |b| {
        b.iter(|| {
            black_box(keypair.encapsulate().unwrap());
        })
    });

    let (ciphertext, _) = keypair.encapsulate().unwrap();
    group.bench_function("decapsulate", |b| {
        b.iter(|| {
            black_box(keypair.decapsulate(&ciphertext).unwrap());
        })
    });

    group.finish();
}

fn bench_blake3_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("blake3");
    
    for size in [64, 256, 1024, 4096, 16384] {
        let data = vec![42u8; size];
        
        group.bench_with_input(BenchmarkId::from_parameter(size), &data, |b, data| {
            b.iter(|| {
                black_box(blake3_hash(data));
            })
        });
    }
    
    group.finish();
}

fn bench_transaction_validation(c: &mut Criterion) {
    let keypair = KeyPair::generate();
    let mut tx = Transaction::new(
        keypair.to_address(),
        Address::new([1u8; 32]),
        Amount::new(1000),
        Amount::new(1),
    );
    
    tx.compute_hash();
    tx.signature = keypair.sign(&tx.serialize());

    c.bench_function("transaction_validate", |b| {
        b.iter(|| {
            black_box(tx.validate().unwrap());
        })
    });
}

fn bench_spiral_computation(c: &mut Criterion) {
    use spirachain_core::{Spiral, SpiralType, SpiralMetadata};
    
    c.bench_function("spiral_create", |b| {
        b.iter(|| {
            let metadata = SpiralMetadata {
                spiral_type: SpiralType::Fibonacci,
                complexity: 85.5,
                self_similarity: 1.618,
                information_density: 3.14159,
            };
            
            black_box(Spiral {
                spiral_type: SpiralType::Fibonacci,
                parameters: vec![1.0, 1.618, 2.0],
                points: Vec::new(),
                metadata,
            });
        })
    });
}

criterion_group!(
    benches,
    bench_block_hashing,
    bench_transaction_signing,
    bench_xmss_operations,
    bench_kyber_operations,
    bench_blake3_hashing,
    bench_transaction_validation,
    bench_spiral_computation,
);
criterion_main!(benches);

