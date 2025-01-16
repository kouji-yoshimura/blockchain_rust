use rand_core::OsRng;
use k256::{
    ecdsa::{
        SigningKey,
        Signature,
        VerifyingKey,
        signature::{
            Signer,
            Verifier,
        },
    },
};
use crate::mylib::hex::{encode_hex, decode_hex};

pub fn generate_private_key() -> String {
    let private_key: SigningKey = SigningKey::random(&mut OsRng);
    encode_hex(&private_key.to_bytes())
}

pub fn public_key_from_private_key(private_key: String) -> Result<String, String> {
    match SigningKey::from_slice(&decode_hex(private_key)) {
        Ok(signing_key) => {
            let verifying_key = signing_key.verifying_key();
            return Ok(encode_hex(&verifying_key.to_encoded_point(false).to_bytes()))
        },
        Err(error) => {
            return Err(error.to_string())
        }
    };
}

pub fn sign(private_key: String, data: String) -> Result<String, String> {
    match SigningKey::from_slice(&decode_hex(private_key)) {
        Ok(signing_key) => {
            let signature: Signature = signing_key.sign(&data.into_bytes());
            return Ok(encode_hex(&signature.to_bytes()))
        },
        Err(error) => {
            return Err(error.to_string())
        }
    };
}

pub fn verify(public_key: String, transaction_id: String, signature: String) -> Result<(), String> {
    let verifying_key = match VerifyingKey::from_sec1_bytes(&decode_hex(public_key)) {
        Err(error) => return Err(error.to_string()),
        Ok(v) => v,
    };
    let signature = match Signature::from_slice(&decode_hex(signature)) {
        Err(error) => return Err(error.to_string()),
        Ok(v) => v,
    };
    match verifying_key.verify(&decode_hex(transaction_id), &signature) {
        Ok(()) => Ok(()),
        Err(error) => Err(error.to_string())
    }
}
