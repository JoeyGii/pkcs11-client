use cryptoki::object::ObjectHandle;
use cryptoki::object::{Attribute, KeyType};
pub mod utility;
use cryptoki::object::ObjectClass;
pub mod cli {
    pub mod cli;
}
pub type KeyPair = (ObjectHandle, ObjectHandle);
impl KeyPairGetter for KeyPair {
    fn get_private_key(&self) -> ObjectHandle {
        self.1
    }
    fn get_public_key(&self) -> ObjectHandle {
        self.0
    }
}
trait KeyPairGetter {
    fn get_public_key(&self) -> ObjectHandle;
    fn get_private_key(&self) -> ObjectHandle;
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
            // ... other attributes as needed
        ]
    }
}
