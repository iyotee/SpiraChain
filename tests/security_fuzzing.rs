use spirachain_core::{Block, Transaction, Hash, Address, Amount};
use spirachain_crypto::{KeyPair, XmssKeyPair, KyberKeyPair};

#[test]
fn fuzz_block_parsing() {
    for i in 0..1000 {
        let mut random_data = vec![0u8; 1024];
        for j in 0..random_data.len() {
            random_data[j] = ((i * j) % 256) as u8;
        }

        let _ = bincode::deserialize::<Block>(&random_data);
    }
}

#[test]
fn fuzz_transaction_parsing() {
    for i in 0..1000 {
        let mut random_data = vec![0u8; 512];
        for j in 0..random_data.len() {
            random_data[j] = ((i + j * 7) % 256) as u8;
        }

        let _ = bincode::deserialize::<Transaction>(&random_data);
    }
}

#[test]
fn fuzz_hash_input() {
    for i in 0..1000 {
        let size = (i % 10000) + 1;
        let data = vec![(i % 256) as u8; size];
        
        let _hash = blake3::hash(&data);
    }
}

#[test]
fn fuzz_address_creation() {
    for i in 0..1000 {
        let mut bytes = [0u8; 32];
        for j in 0..32 {
            bytes[j] = ((i + j) % 256) as u8;
        }
        
        let _address = Address::new(bytes);
        let _str = _address.to_string();
    }
}

#[test]
fn fuzz_amount_operations() {
    for i in 0..1000 {
        let amount1 = Amount::new(i as u128 * 1000000);
        let amount2 = Amount::new((i + 1) as u128 * 1000000);
        
        let _is_greater = amount1 > amount2;
        let _is_equal = amount1 == amount2;
        let _display = format!("{}", amount1);
    }
}

#[test]
fn stress_test_keypair_generation() {
    for _ in 0..100 {
        let _keypair = KeyPair::generate();
    }
}

#[test]
fn stress_test_xmss_signatures() {
    let mut keypair = XmssKeyPair::generate().unwrap();
    
    for i in 0..50 {
        let message = format!("Message {}", i);
        let signature = keypair.sign(message.as_bytes()).unwrap();
        assert!(keypair.verify(message.as_bytes(), &signature));
    }
}

#[test]
fn stress_test_kyber_operations() {
    for _ in 0..50 {
        let keypair = KyberKeyPair::generate().unwrap();
        let (ciphertext, shared_secret1) = keypair.encapsulate().unwrap();
        let shared_secret2 = keypair.decapsulate(&ciphertext).unwrap();
        
        assert_eq!(shared_secret1.as_bytes(), shared_secret2.as_bytes());
    }
}

#[test]
fn test_concurrent_operations() {
    use std::thread;
    use std::sync::Arc;
    
    let handles: Vec<_> = (0..10).map(|i| {
        thread::spawn(move || {
            for j in 0..100 {
                let keypair = KeyPair::generate();
                let message = format!("Thread {} - Message {}", i, j);
                let _signature = keypair.sign(message.as_bytes());
            }
        })
    }).collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_memory_safety() {
    let mut blocks = Vec::new();
    
    for i in 0..1000 {
        let mut block = Block::new(Hash::zero(), i);
        
        for j in 0..10 {
            let tx = Transaction::new(
                Address::new([j as u8; 32]),
                Address::new([(j + 1) as u8; 32]),
                Amount::new(1000),
                Amount::new(1),
            );
            block.transactions.push(tx);
        }
        
        blocks.push(block);
    }
    
    assert_eq!(blocks.len(), 1000);
}

