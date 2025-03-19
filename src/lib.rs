use bech32::{ToBase32, Variant};
use secp256k1::{KeyPair, Secp256k1, XOnlyPublicKey};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self},
    Arc,
};
use std::thread;

pub struct NostrKeyGenerator {
    thread_count: u32,
}

impl NostrKeyGenerator {
    pub fn new(thread_count: u32) -> Self {
        Self { thread_count }
    }

    pub fn thread_count(&self) -> u32 {
        self.thread_count
    }

    pub fn generate_key() -> KeyPair {
        let secp = Secp256k1::new();
        let mut rng = rand::rngs::OsRng::default();
        KeyPair::new(&secp, &mut rng)
    }

    pub fn to_npub(pubkey: &XOnlyPublicKey) -> String {
        bech32::encode("npub", pubkey.serialize().to_base32(), Variant::Bech32).unwrap()
    }

    pub fn to_nsec(keypair: &KeyPair) -> String {
        bech32::encode(
            "nsec",
            keypair.secret_key().secret_bytes().to_base32(),
            Variant::Bech32,
        )
        .unwrap()
    }

    pub fn find_vanity_key(
        &self,
        primary_prefix: &str,
        additional_prefixes: Option<&[String]>,
    ) -> (String, String) {
        let (sender, receiver) = mpsc::channel();
        let found = Arc::new(AtomicBool::new(false));

        let mut handles = vec![];
        for _ in 0..self.thread_count {
            let sender = sender.clone();
            let found = found.clone();
            let primary_prefix = primary_prefix.to_string();
            let thread_additional = additional_prefixes
                .map(|prefixes| prefixes.iter().map(|s| s.clone()).collect::<Vec<String>>());

            let handle = thread::spawn(move || {
                while !found.load(Ordering::Relaxed) {
                    let keypair = Self::generate_key();
                    let (pubkey, _) = keypair.x_only_public_key();

                    let npub = Self::to_npub(&pubkey);
                    let nsec = Self::to_nsec(&keypair);

                    if let Some(ref additional) = thread_additional {
                        for add_prefix in additional {
                            if npub.starts_with(&format!("npub1{}", add_prefix)) {
                                println!("\nFound additional match!");
                                println!("Public Key (npub): {}", npub);
                                println!("Private Key (nsec): {}", nsec);
                                println!("\n Continuing search for primary prefix...");
                            }
                        }
                    }

                    if npub.starts_with(&format!("npub1{}", primary_prefix)) {
                        let (derived_pubkey, _) = keypair.x_only_public_key();
                        assert_eq!(pubkey, derived_pubkey, "Key pair mismatch!");

                        found.store(true, Ordering::Relaxed);
                        let _ = sender.send((npub, nsec));
                        break;
                    }
                }
            });
            handles.push(handle);
        }
        let result = receiver.recv().unwrap();
        for handle in handles {
            let _ = handle.join();
        }
        result
    }
}
