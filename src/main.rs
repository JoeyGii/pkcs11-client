use clap::Parser;
use cryptoki::context::CInitializeArgs;
use cryptoki::context::Pkcs11;
use cryptoki::error::Error;
use cryptoki::mechanism::Mechanism;
use cryptoki::object::ObjectClass;
use cryptoki::object::ObjectHandle;
use cryptoki::object::{Attribute, KeyType};
use pkcs11_client::cli::cli::Args;
use pkcs11_client::utility::*;
use pkcs11_client::KeyPairUtility;
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
fn main() -> Result<(), Error> {
    // let args = Args::parse();

    let pkcs11 = Pkcs11::new(LIBRARY_PATH).expect("Expected working path to PKCS11 lib");
    pkcs11.initialize(CInitializeArgs::OsThreads).unwrap();
    let slots = pkcs11.get_all_slots()?;

    //TODO: i need to find how i would get a relevant slot.
    let slot = slots[0];
    // Open a session with the HSM
    let session = pkcs11.open_rw_session(slot)?;

    let mechanism = &Mechanism::AesKeyGen;
    let keypair_utility = KeyPairUtility::default();

    let key_pair: KeyPair = session.generate_key_pair(
        mechanism,
        &keypair_utility.generate_pub_key_template(),
        &keypair_utility.generate_priv_key_template(),
    )?;

    // Sign some data
    let data = "this week i tried to...".as_bytes();
    let signature = session.sign(mechanism, key_pair.get_private_key(), data)?;

    // Verify the signature
    let verified = session
        .verify(&mechanism, key_pair.get_public_key(), data, &signature)
        .is_ok();
    println!("Signature verified: {}", verified);

    Ok(())
}
