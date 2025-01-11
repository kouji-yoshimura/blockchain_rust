use rand_core::OsRng;
use k256::ecdsa::{
    SigningKey,
    Signature,
    VerifyingKey,
    signature::Signer
};

pub fn public_key_from(private_key: String) -> Result<String, String> {
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

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|n| format!("{:02X}", n)).collect::<String>()
}

fn decode_hex(hex: String) -> Vec<u8> {
    (0..hex.as_str().len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>()
}

pub struct KeyPair {
    name: String,
    private_key: SigningKey,
    public_key: VerifyingKey,
}

impl KeyPair {
    pub fn name(&self) -> String { self.name.clone() }

    pub fn public_key(&self) -> String {
        let bytes = self.public_key.to_encoded_point(false).to_bytes();
        encode_hex(&bytes)
    }

    pub fn private_key(&self) -> String {
        let bytes = self.private_key.to_bytes();
        encode_hex(&bytes)
    }

    pub fn generate(name: String) -> Self{
        let private_key: SigningKey = SigningKey::random(&mut OsRng);

        let binding = private_key.clone();
        let public_key: &VerifyingKey = binding.verifying_key();

        KeyPair {
            name,
            private_key,
            public_key: *public_key,
        }
    }
}
