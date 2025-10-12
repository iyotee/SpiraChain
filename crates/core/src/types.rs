use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(data: [u8; 32]) -> Self {
        Self(data)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, &'static str> {
        if slice.len() != 32 {
            return Err("Hash must be 32 bytes");
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(slice);
        Ok(Self(arr))
    }

    pub fn zero() -> Self {
        Self([0u8; 32])
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

impl From<blake3::Hash> for Hash {
    fn from(hash: blake3::Hash) -> Self {
        Self(*hash.as_bytes())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PiCoordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub t: f64,
}

impl PiCoordinate {
    pub fn new(x: f64, y: f64, z: f64, t: f64) -> Self {
        Self { x, y, z, t }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            t: 0.0,
        }
    }

    pub fn distance(&self, other: &PiCoordinate) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dt = self.t - other.t;
        (dx * dx + dy * dy + dz * dz + dt * dt).sqrt()
    }
    
    pub fn from_hash_timestamp(entity_hash: &[u8], timestamp: u64, nonce: u64) -> Self {
        let hash = blake3::hash(entity_hash);
        let hash_bytes = hash.as_bytes();
        
        let x = f64::from_le_bytes([
            hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
            hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
        ]);
        let y = f64::from_le_bytes([
            hash_bytes[8], hash_bytes[9], hash_bytes[10], hash_bytes[11],
            hash_bytes[12], hash_bytes[13], hash_bytes[14], hash_bytes[15],
        ]);
        let z = f64::from_le_bytes([
            hash_bytes[16], hash_bytes[17], hash_bytes[18], hash_bytes[19],
            hash_bytes[20], hash_bytes[21], hash_bytes[22], hash_bytes[23],
        ]);
        let t = (timestamp ^ nonce) as f64;
        
        Self { x, y, z, t }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address([u8; 32]);

impl Address {
    pub fn new(data: [u8; 32]) -> Self {
        Self(data)
    }

    pub fn from_slice(slice: &[u8]) -> Result<Self, &'static str> {
        if slice.len() != 32 {
            return Err("Address must be 32 bytes");
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(slice);
        Ok(Self(arr))
    }

    pub fn zero() -> Self {
        Self([0u8; 32])
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(u128);

impl Amount {
    pub fn new(value: u128) -> Self {
        Self(value)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn qbt(value: u64) -> Self {
        Self(value as u128 * 10u128.pow(crate::TOKEN_DECIMALS as u32))
    }

    pub fn from_millis(value: u64) -> Self {
        Self(value as u128 * 10u128.pow((crate::TOKEN_DECIMALS - 3) as u32))
    }

    pub fn value(&self) -> u128 {
        self.0
    }

    pub fn checked_add(&self, other: Amount) -> Option<Amount> {
        self.0.checked_add(other.0).map(Amount)
    }

    pub fn checked_sub(&self, other: Amount) -> Option<Amount> {
        self.0.checked_sub(other.0).map(Amount)
    }

    pub fn checked_mul(&self, factor: u64) -> Option<Amount> {
        self.0.checked_mul(factor as u128).map(Amount)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let qbt = self.0 / 10u128.pow(crate::TOKEN_DECIMALS as u32);
        let fraction = self.0 % 10u128.pow(crate::TOKEN_DECIMALS as u32);
        write!(f, "{}.{:0width$} QBT", qbt, fraction, width = crate::TOKEN_DECIMALS as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpiralType {
    Archimedean = 0,
    Logarithmic = 1,
    Fibonacci = 2,
    Fermat = 3,
    Ramanujan = 4,
    Custom = 99,
}

impl fmt::Display for SpiralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpiralType::Archimedean => write!(f, "Archimedean"),
            SpiralType::Logarithmic => write!(f, "Logarithmic"),
            SpiralType::Fibonacci => write!(f, "Fibonacci"),
            SpiralType::Fermat => write!(f, "Fermat"),
            SpiralType::Ramanujan => write!(f, "Ramanujan"),
            SpiralType::Custom => write!(f, "Custom"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Person = 0,
    Organization = 1,
    Location = 2,
    Concept = 3,
    Event = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentType {
    Transfer = 0,
    ContractCall = 1,
    DataStorage = 2,
    Governance = 3,
    Social = 4,
}
