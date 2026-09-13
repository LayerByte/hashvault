use std::env;
use std::fs;
use std::net::Ipv4Addr;
use std::path::Path;
use std::process;

const PROJECT_NAME: &str = "Hashvault";
const PROJECT_FOCUS: &str = "Fast file hashing utility.";

fn entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut counts = [0usize; 256];
    for byte in data {
        counts[*byte as usize] += 1;
    }
    counts
        .iter()
        .filter(|count| **count > 0)
        .map(|count| {
            let p = *count as f64 / data.len() as f64;
            -p * p.log2()
        })
        .sum()
}

fn fnv1a(data: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in data {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn inspect_file(path: &Path) -> Result<(), String> {
    let data = fs::read(path).map_err(|error| format!("could not read file: {error}"))?;
    let metadata = fs::metadata(path).map_err(|error| format!("could not read metadata: {error}"))?;
    println!("Project: {PROJECT_NAME}");
    println!("Focus: {PROJECT_FOCUS}");
    println!("Path: {}", path.display());
    println!("Size: {} bytes", metadata.len());
    println!("Entropy: {:.4}", entropy(&data));
    println!("FNV-1a fingerprint: {:016x}", fnv1a(&data));
    let header: Vec<String> = data.iter().take(16).map(|byte| format!("{byte:02x}")).collect();
    println!("Header: {}", header.join(" "));
    Ok(())
}

fn strings(path: &Path) -> Result<(), String> {
    let data = fs::read(path).map_err(|error| format!("could not read file: {error}"))?;
    let mut current = String::new();
    for byte in data {
        if byte.is_ascii_graphic() || byte == b' ' {
            current.push(byte as char);
        } else {
            if current.len() >= 4 {
                println!("{current}");
            }
            current.clear();
        }
    }
    if current.len() >= 4 {
        println!("{current}");
    }
    Ok(())
}

fn cidr(cidr: &str) -> Result<(), String> {
    let (ip_text, prefix_text) = cidr.split_once('/').ok_or("CIDR must look like 192.168.1.0/24")?;
    let ip: Ipv4Addr = ip_text.parse().map_err(|_| "invalid IPv4 address")?;
    let prefix: u32 = prefix_text.parse().map_err(|_| "invalid prefix")?;
    if prefix > 32 {
        return Err("prefix must be 0-32".to_string());
    }
    let raw = u32::from(ip);
    let mask = if prefix == 0 { 0 } else { u32::MAX << (32 - prefix) };
    let network = raw & mask;
    let broadcast = network | !mask;
    println!("Network: {}", Ipv4Addr::from(network));
    println!("Broadcast: {}", Ipv4Addr::from(broadcast));
    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{PROJECT_NAME} - {PROJECT_FOCUS}");
        println!("Usage: {} <file|cidr> [--strings]", args[0]);
        return Ok(());
    }
    if args[1].contains('/') && args[1].contains('.') && !Path::new(&args[1]).exists() {
        cidr(&args[1])
    } else if args.iter().any(|arg| arg == "--strings") {
        strings(Path::new(&args[1]))
    } else {
        inspect_file(Path::new(&args[1]))
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}
