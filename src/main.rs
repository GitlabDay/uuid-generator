use clap::{Parser, ValueEnum};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::{Uuid, Timestamp, NoContext};

/// Simple UUID generator supporting v4 and v7 and multiple output formats.
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// UUID type to generate: v4 or v7
    #[arg(short, long = "type", value_enum, default_value = "v4")]
    r#type: UuidType,

    /// Number of UUIDs to generate
    #[arg(short, long, default_value_t = 1)]
    count: usize,

    /// Output format
    #[arg(short, long, value_enum, default_value = "all")]
    format: OutputFormat,

    /// Quiet mode: suppress header and line labels, print only the raw UUID value(s)
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum UuidType {
    V4,
    V7,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// lowercase with dashes
    LowerDash,
    /// UPPERCASE with dashes
    UpperDash,
    /// lowercase without dashes
    LowerNoDash,
    /// UPPERCASE without dashes
    UpperNoDash,
    /// print all formats
    All,
}

fn fmt_variants(u: &Uuid) -> (String, String, String, String) {
    let lower_dash = u.to_string(); // lowercase with dashes
    let upper_dash = lower_dash.to_uppercase();
    let lower_nodash = lower_dash.replace('-', "");
    let upper_nodash = lower_nodash.to_uppercase();
    (lower_dash, upper_dash, lower_nodash, upper_nodash)
}

/// Build a v7 Timestamp from current system time.
/// Uses `NoContext` for the clock-sequence when you don't need persistent state.
fn current_v7_timestamp() -> Timestamp {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before UNIX_EPOCH");

    let secs = dur.as_secs();
    let nanos = dur.subsec_nanos();

    // For uuid crate versions like 1.19.0 the signature is:
    // Timestamp::from_unix(context, secs, nanos)
    Timestamp::from_unix(NoContext, secs, nanos)
}

fn main() {
    let args = Args::parse();

    for i in 0..args.count {
        let u = match args.r#type {
            UuidType::V4 => Uuid::new_v4(),
            UuidType::V7 => {
                let ts = current_v7_timestamp();
                Uuid::new_v7(ts)
            }
        };

        let (lower_dash, upper_dash, lower_nodash, upper_nodash) = fmt_variants(&u);

        if args.quiet {
            // Quiet: print only a single raw UUID string per item.
            // If the selected format is `All`, choose `lower-dash` as the canonical quiet output.
            let out = match args.format {
                OutputFormat::LowerDash => lower_dash,
                OutputFormat::UpperDash => upper_dash,
                OutputFormat::LowerNoDash => lower_nodash,
                OutputFormat::UpperNoDash => upper_nodash,
                OutputFormat::All => lower_dash,
            };
            println!("{}", out);
        } else {
            // Non-quiet: print header and labeled lines as requested.
            match args.r#type {
                UuidType::V4 => println!("UUIDv4"),
                UuidType::V7 => println!("UUIDv7"),
            }
            println!();

            match args.format {
                OutputFormat::LowerDash => println!("lower-dash: {}", lower_dash),
                OutputFormat::UpperDash => println!("UPPER-dash: {}", upper_dash),
                OutputFormat::LowerNoDash => println!("lower-nodash: {}", lower_nodash),
                OutputFormat::UpperNoDash => println!("UPPER-nodash: {}", upper_nodash),
                OutputFormat::All => {
                    println!("lower-dash:   {}", lower_dash);
                    println!("UPPER-dash:   {}", upper_dash);
                    println!("lower-nodash: {}", lower_nodash);
                    println!("UPPER-nodash: {}", upper_nodash);
                }
            }

            // Separate multiple UUID outputs with a blank line
            if i + 1 < args.count {
                println!();
            }
        }
    }
}
