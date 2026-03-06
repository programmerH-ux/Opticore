use sha2::{Sha256, Digest};
use sysinfo::System;
use pyo3::prelude::*;
use ed25519_dalek::{Signature, VerifyingKey, Verifier};
use base64::{engine::general_purpose, Engine as _};
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::NaiveDate;

fn machine_fingerprint() -> String {
    use sha2::{Sha256, Digest};
    use sysinfo::System;

    let mut system = System::new_all();
    system.refresh_all();

    let cpu_brand = system.global_cpu_info().brand();
    let total_memory = system.total_memory();
    let os = System::name().unwrap_or_default();

    let raw = format!("{}-{}-{}", cpu_brand, total_memory, os);

    let mut hasher = Sha256::new();
    hasher.update(raw.as_bytes());
    let result = hasher.finalize();

    hex::encode(result)
}

// License activation state
static LICENSE_VALID: AtomicBool = AtomicBool::new(false);

const PUBLIC_KEY_BYTES: [u8; 32] = [222, 191, 117, 51, 124, 170, 60, 9, 49, 153, 86, 110, 23, 231, 97, 79, 47, 164, 30, 35, 162, 56, 53, 63, 82, 249, 213, 84, 165, 121, 247, 86];

#[derive(Serialize, Deserialize)]
struct LicensePayload {
    email: String,
    expiry: String,     // "YYYY-MM-DD" or "" for lifetime
    plan: String,       // "monthly", "yearly", "lifetime"
    machine: String,    // machine fingerprint
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

    // Verify license signature
    public_key
        .verify(&payload_bytes, &signature)
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("License verification failed"))?;

    // Deserialize payload
    let payload: LicensePayload = serde_json::from_slice(&payload_bytes)
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("Invalid payload"))?;

    // Check expiry date
    let expiry_date = NaiveDate::parse_from_str(&payload.expiry, "%Y-%m-%d")
        .map_err(|_| pyo3::exceptions::PyPermissionError::new_err("Invalid expiry format"))?;

    let today = chrono::Utc::now().date_naive();

    if today > expiry_date {
        return Err(pyo3::exceptions::PyPermissionError::new_err(
            "License expired",
        ));
    }

    // ✅ MACHINE BINDING CHECK
    let current_machine = machine_fingerprint();
    if payload.machine != current_machine {
        return Err(pyo3::exceptions::PyPermissionError::new_err(
            "License is not valid for this machine",
        ));
    }

    // License is valid
    LICENSE_VALID.store(true, Ordering::Relaxed);
    Ok(())
}

#[pyfunction(name = "machine_id")]
pub fn machine_id_py() -> PyResult<String> {
    Ok(machine_fingerprint())
}
