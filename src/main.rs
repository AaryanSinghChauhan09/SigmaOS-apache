use anyhow::Result;
use clap::Parser;
use apache::apache;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    production: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = apache::Config { 
        production_mode: cli.production, 
        ..Default::default() 
    };
    let mut component = apache::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-apache running");
    Ok(())
}
