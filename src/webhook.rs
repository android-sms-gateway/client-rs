use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Verifies an HMAC-SHA256 webhook signature.
///
/// Use this function to verify that an incoming webhook request was sent by
/// the SMSGate device. The verification uses constant-time
/// comparison to prevent timing attacks.
///
/// # Algorithm
///
/// 1. Concatenate the raw request body with the `X-Timestamp` value
/// 2. Compute HMAC-SHA256 using the signing key
/// 3. Compare with `X-Signature` header using constant-time comparison
///
/// # Example
///
/// ```no_run
/// use android_sms_gateway::webhook::verify_signature;
///
/// let secret_key = "your-signing-key";
/// let body = r#"{"event":"sms:received","payload":{}}"#;
/// let timestamp = "1700000000";
/// let signature = "abc123...";
///
/// if verify_signature(secret_key, body, timestamp, signature) {
///     println!("Signature is valid");
/// }
/// ```
pub fn verify_signature(secret_key: &str, body: &str, timestamp: &str, signature: &str) -> bool {
    let mut mac = match Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };

    mac.update(body.as_bytes());
    mac.update(timestamp.as_bytes());

    match hex::decode(signature) {
        Ok(sig_bytes) => mac.verify_slice(&sig_bytes).is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_valid_signature() {
        let secret_key = "my-secret-key";
        let body = r#"{"event":"sms:received","payload":{"message":"Hello"}}"#;
        let timestamp = "1700000000";

        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
        let message = format!("{}{}", body, timestamp);
        mac.update(message.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        assert!(verify_signature(secret_key, body, timestamp, &signature));
    }

    #[test]
    fn test_verify_invalid_signature() {
        let secret_key = "my-secret-key";
        let body = r#"{"event":"sms:received","payload":{"message":"Hello"}}"#;
        let timestamp = "1700000000";

        assert!(!verify_signature(secret_key, body, timestamp, "deadbeef"));
    }

    #[test]
    fn test_verify_wrong_key() {
        let secret_key = "my-secret-key";
        let body = r#"{"event":"sms:received"}"#;
        let timestamp = "1700000000";

        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
        let message = format!("{}{}", body, timestamp);
        mac.update(message.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        assert!(!verify_signature("wrong-key", body, timestamp, &signature));
    }

    #[test]
    fn test_verify_wrong_timestamp() {
        let secret_key = "my-secret-key";
        let body = r#"{"event":"sms:received"}"#;

        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
        let message = format!("{}{}", body, "1700000000");
        mac.update(message.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        assert!(!verify_signature(
            secret_key,
            body,
            "1700000001",
            &signature
        ));
    }

    #[test]
    fn test_verify_empty_values() {
        assert!(!verify_signature("", "", "", ""));
        assert!(!verify_signature("key", "", "", "sig"));
    }

    #[test]
    fn test_verify_replay_detection() {
        let secret_key = "test-key";
        let body = "payload";
        let old_timestamp = "1000";
        let new_timestamp = "2000";

        let mut mac = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).unwrap();
        let message = format!("{}{}", body, old_timestamp);
        mac.update(message.as_bytes());
        let old_sig = hex::encode(mac.finalize().into_bytes());

        assert!(verify_signature(secret_key, body, old_timestamp, &old_sig));
        assert!(!verify_signature(secret_key, body, new_timestamp, &old_sig));
    }
}
