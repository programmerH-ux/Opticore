use pyo3::prelude::*;
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize};
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::NaiveDate;
use crate::license::ensure_pro()?;

// License activation state
static LICENSE_VALID: AtomicBool = AtomicBool::new(false);

const PUBLIC_KEY_BYTES: [u8; 32] = [104, 255, 73, 240, 99, 190, 83, 244, 72, 127, 161, 180, 132, 103, 2, 181, 186, 124, 181, 13, 212, 142, 116, 155, 22, 212, 69, 66, 33, 179, 149, 153];

#[derive(Deserialize)]
struct LicensePayload {
    email: String,
    expiry: String,
    plan: String,
}

// Called internally by Pro features
pub fn ensure_pro() -> PyResult<()> {
    if !LICENSE_VALID.load(Ordering::Relaxed) {
        return Err(pyo3::exceptions::PyPermissionError::new_err(
            "This feature requires an active OptiEngine Pro license.",
        ));
    }
    Ok(())
}

#[pyfunction(name = "activate_license")]
pub fn activate_license_py(key: &str) -> PyResult<()> {
    let parts: Vec<&str> = key.split(".").collect();
    if parts.len() != 2 {
        return Err(pyo3::exceptions::PyPermissionError::new_err(
            "Invalid license format",
        ));
    }

    let payload_encoded = parts[0];
    let signature_encoded = parts[1];

    let payload_bytes = general_purpose::STANDARD
        .decode(payload_encoded)
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("Invalid payload encoding"))?;

    let signature_bytes = general_purpose::STANDARD
        .decode(signature_encoded)
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("Invalid signature encoding"))?;

    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("Invalid signature"))?;

    let public_key = VerifyingKey::from_bytes(&PUBLIC_KEY_BYTES)
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("Invalid public key"))?;

    public_key
        .verify(&payload_bytes, &signature)
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("License verification failed"))?;

    // Check expiry date
    let payload: LicensePayload = serde_json::from_slice(&payload_bytes)
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("Invalid payload"))?;

    let expiry_date = NaiveDate::parse_from_str(&payload.expiry, "%Y-%m-%d")
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("Invalid expiry format"))?;

    let today = chrono::Utc::now().date_naive();

    if today > expiry_date {
        return Err(pyo3::exceptions::PyPermissionError::new_err(
            "License expired",
        ));
    }

    LICENSE_VALID.store(true, Ordering::Relaxed);
    Ok(())
}
