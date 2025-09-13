use ed25519_dalek::{Keypair, Signature, Signer, PublicKey, Verifier};
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::Utc;
use base64::{engine::general_purpose, Engine as _};

/// Wipe certificate structure
#[derive(Serialize, Deserialize, Debug)]
pub struct WipeCertificate {
    pub certificate_id: String,
    pub device: String,
    pub method: String,
    pub timestamp: String,
    pub status: String,
    pub public_key: String,
    pub signature: Option<String>,
}

impl WipeCertificate {
    pub fn new(device: &str, method: &str) -> Self {
        WipeCertificate {
            certificate_id: Uuid::new_v4().to_string(),
            device: device.to_string(),
            method: method.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            status: "success".to_string(),
            public_key: "".to_string(),
            signature: None,
        }
    }
}

/// Generate ephemeral Ed25519 keypair
pub fn generate_keypair() -> Keypair {
    let mut csprng = OsRng;
    Keypair::generate(&mut csprng)
}

/// Sign a certificate
pub fn sign_certificate(mut cert: WipeCertificate, keypair: &Keypair) -> WipeCertificate {
    cert.public_key = general_purpose::STANDARD.encode(keypair.public.to_bytes());

    let mut value = serde_json::to_value(&cert).unwrap();
    if let serde_json::Value::Object(ref mut map) = value {
        map.remove("signature");
    }
    let bytes = serde_json::to_vec(&value).unwrap();

    let sig: Signature = keypair.sign(&bytes);
    cert.signature = Some(general_purpose::STANDARD.encode(sig.to_bytes()));

    cert
}

/// Verify a signed certificate
pub fn verify_certificate(cert: &WipeCertificate) -> bool {
    if cert.signature.is_none() {
        return false;
    }

    let sig_bytes = general_purpose::STANDARD
        .decode(cert.signature.as_ref().unwrap())
        .unwrap();
    let pk_bytes = general_purpose::STANDARD
        .decode(&cert.public_key)
        .unwrap();

    let public_key = PublicKey::from_bytes(&pk_bytes).unwrap();
    let sig = Signature::from_bytes(&sig_bytes).unwrap();

    let mut value = serde_json::to_value(cert).unwrap();
    if let serde_json::Value::Object(ref mut map) = value {
        map.remove("signature");
    }
    let bytes = serde_json::to_vec(&value).unwrap();

    public_key.verify(&bytes, &sig).is_ok()
}
