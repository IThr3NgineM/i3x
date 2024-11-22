use structopt::StructOpt;
use anyhow::Result;
use ic_cdk::spawn;

mod commands;
mod utils;
mod i3m_project;
mod registry;

#[derive(StructOpt)]
#[structopt(name = "i3x", about = "I3M Engine's decentralized CLI")]
enum I3X {
    #[structopt(about = "Registry management (reg)")]
    Reg(commands::Registry),

    #[structopt(about = "Project management (proj)")]
    Proj(commands::I3MProject),
}

fn main() -> Result<()> {
    let opt = I3X::from_args();

    spawn(async move {
        if let Err(e) = run_async(opt).await {
            eprintln!("Error: {:?}", e);
        }
    });

    Ok(())
}

// Asynchronous function to handle parsed commands
async fn run_async(opt: I3X) -> Result<()> {
    match opt {
        I3X::Reg(cmd) => commands::handle_registry_command(cmd).await?,
        I3X::Proj(cmd) => commands::handle_project_command(cmd)?,
    }
    Ok(())
}
