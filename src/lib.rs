use cryptoki::object::ObjectHandle;
use cryptoki::object::{Attribute, KeyType};
pub mod utility;
use cryptoki::object::ObjectClass;
pub mod cli {
    pub mod cli;
}

#[derive(Default)]
pub struct KeyPairUtility;

impl KeyPairUtility {
    pub fn generate_pub_key_template(&self) -> Vec<Attribute> {
        vec![
            Attribute::Class(ObjectClass::PUBLIC_KEY),
            Attribute::KeyType(KeyType::SHA256_HMAC),
            Attribute::Token(true),
            Attribute::ValueLen(2048.into()),
            Attribute::Encrypt(true),
        ]
    }

    pub fn generate_priv_key_template(&self) -> Vec<Attribute> {
        vec![
            Attribute::Class(ObjectClass::PRIVATE_KEY),
            Attribute::KeyType(KeyType::SHA256_HMAC),
            Attribute::Token(true),
            Attribute::ValueLen(2048.into()),
            Attribute::Decrypt(true),
            Attribute::Sign(true),
        ]
    }
}
