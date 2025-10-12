use spirachain_core::Hash;

pub fn blake3_hash(data: &[u8]) -> Hash {
    blake3::hash(data).into()
}

pub fn blake3_keyed_hash(key: &[u8; 32], data: &[u8]) -> Hash {
    blake3::keyed_hash(key, data).into()
}

pub fn blake3_derive_key(context: &str, key_material: &[u8]) -> [u8; 32] {
    blake3::derive_key(context, key_material)
}

pub fn double_hash(data: &[u8]) -> Hash {
    let first = blake3_hash(data);
    blake3_hash(first.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_hash() {
        let data = b"test data";
        let hash1 = blake3_hash(data);
        let hash2 = blake3_hash(data);
        
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_different_data_different_hash() {
        let data1 = b"test data 1";
        let data2 = b"test data 2";
        let hash1 = blake3_hash(data1);
        let hash2 = blake3_hash(data2);
        
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_double_hash() {
        let data = b"test data";
        let hash = double_hash(data);
        
        assert_ne!(hash, Hash::zero());
    }
}

