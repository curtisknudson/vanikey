use clap::Parser;
use vanikey::NostrKeyGenerator;

#[derive(Parser)]
#[command(
    name = "vanikey",
    about = "Generate Nostr vanity keys",
    long_about = "Generate Nostr vanity key(s) to have your personalized Nostr npub",
    version,
    author = "Yggr"
)]

struct Cli {
    #[arg(
        value_name = "PREFIX",
        required = true,
        help = "The prefix to search for in the npub (allowed characters: 023456789acdefghjklmnpqrstuvwxyz)"
    )]
    prefix: String,

    #[arg(short, long, default_value_t = 4, help = "Number of threads to use")]
    threads: u32,

    #[arg(short, long, num_args = 1.., value_delimiter = ',')]
    additional: Vec<String>,
}

fn validate_prefix(prefix: &str) -> Result<(), String> {
    if prefix.is_empty() {
        return Err("Prefix cannot be empty".to_string());
    }

    // Bech32 allowed characters
    const ALLOWED_CHARS: &str = "023456789acdefghjklmnpqrstuvwxyz";

    for c in prefix.chars() {
        if !ALLOWED_CHARS.contains(c) {
            return Err(format!(
                "Invalid character '{}' in prefix. Allowed characters are: {}",
                c, ALLOWED_CHARS
            ));
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    // Validate main prefix
    if let Err(e) = validate_prefix(&cli.prefix) {
        eprintln!("Error in main prefix: {}", e);
        std::process::exit(1);
    }

    // Validate additional prefixes if any
    let additional_prefixes = if !cli.additional.is_empty() {
        for prefix in &cli.additional {
            if let Err(e) = validate_prefix(prefix) {
                eprintln!("Error in additional prefix '{}': {}", prefix, e);
                std::process::exit(1);
            }
        }
        Some(cli.additional.as_slice())
    } else {
        None
    };

    println!(
        "Searching for npub1{} using {} threads...",
        cli.prefix, cli.threads
    );

    if let Some(additional) = additional_prefixes {
        println!("Also looking for: {}", additional.join(", "));
    }

    let generator = NostrKeyGenerator::new(cli.threads);

    let (npub, nsec) = generator.find_vanity_key(&cli.prefix, additional_prefixes);

    println!("Found matching key!");
    println!("Public Key (npub): {}", npub);
    println!("Private Key (nsec): {}", nsec);
}
