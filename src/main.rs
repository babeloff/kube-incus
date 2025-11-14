use clap::Parser;

/// Kubeincus - A bridge enabling KubeVirt to manage system-containers with Incus/LXD
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("Kubeincus starting in verbose mode...");
    }

    println!("Kubeincus v{}", env!("CARGO_PKG_VERSION"));
    println!("A bridge enabling KubeVirt to manage system-containers with Incus/LXD");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}
