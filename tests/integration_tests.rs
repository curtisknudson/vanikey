use bech32::FromBase32;
use secp256k1::{KeyPair, Secp256k1, XOnlyPublicKey};
use vanikey::NostrKeyGenerator;

#[test]
fn test_generator_creation() {
    let generator = NostrKeyGenerator::new(4);
    assert_eq!(generator.thread_count(), 4);
}

#[test]
fn test_key_generation() {
    let keypair = NostrKeyGenerator::generate_key();
    let (pubkey, _) = XOnlyPublicKey::from_keypair(&keypair);
    assert!(!pubkey.serialize().is_empty());
}

#[test]
fn test_npub_conversion() {
    let keypair = NostrKeyGenerator::generate_key();
    let (pubkey, _) = XOnlyPublicKey::from_keypair(&keypair);
    let npub = NostrKeyGenerator::to_npub(&pubkey);

    assert!(npub.starts_with("npub1"));
    assert_eq!(npub.len(), 63);
}

#[test]
fn test_vanity_generation() {
    let generator = NostrKeyGenerator::new(4);
    let prefix = "test";
    let (npub, _) = generator.find_vanity_key(prefix, None);
    assert!(npub.starts_with(&format!("npub1{}", prefix)));
}

#[test]
fn test_multiple_threads() {
    let generator = NostrKeyGenerator::new(8);
    let prefix = "a";
    let (npub, _) = generator.find_vanity_key(prefix, None);
    assert!(npub.starts_with(&format!("npub1{}", prefix)));
}

#[test]
fn test_generated_key_format() {
    let generator = NostrKeyGenerator::new(1);
    let (npub, nsec) = generator.find_vanity_key("a", None);

    // Decode the nsec to get the private key
    let (_, nsec_data, _) = bech32::decode(&nsec).unwrap();
    let secret_bytes = Vec::<u8>::from_base32(&nsec_data).unwrap();
    let secret_key = secp256k1::SecretKey::from_slice(&secret_bytes).unwrap();

    // Generate public key from private key
    let secp = Secp256k1::new();
    let keypair = KeyPair::from_secret_key(&secp, &secret_key);
    let (derived_pubkey, _) = keypair.x_only_public_key();
    let derived_npub = NostrKeyGenerator::to_npub(&derived_pubkey);

    // Verify the keys match
    assert_eq!(npub, derived_npub);
}

#[test]
fn test_with_additional_prefixes() {
    let generator = NostrKeyGenerator::new(4);
    let primary_prefix = "test";
    let additional_prefixes = vec!["abc".to_string(), "xyz".to_string()];
    let (npub, _) = generator.find_vanity_key(primary_prefix, Some(&additional_prefixes));

    // The final result should match the primary prefix
    assert!(npub.starts_with(&format!("npub1{}", primary_prefix)));
}
