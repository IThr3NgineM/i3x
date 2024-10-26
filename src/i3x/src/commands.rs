use structopt::StructOpt;
use anyhow::Result;
use crate::registry;
use crate::i3m_project;


#[derive(StructOpt)]
pub enum Registry {
    #[structopt(about = "Add a module (add)")]
    Add {
        #[structopt(short, long)]
        module: String,
        #[structopt(short, long)]
        version: String,
        #[structopt(short, long)]
        deps: Vec<String>,
    },

    #[structopt(about = "List module versions (list)")]
    List {
        #[structopt(short, long)]
        module: String,
    },

    #[structopt(about = "Install module (install)")]
    Install {
        #[structopt(short, long)]
        module: String,
        #[structopt(short, long)]
        version: Option<String>,
    },

    #[structopt(about = "Update module (up)")]
    Up {
        #[structopt(short, long)]
        module: String,
    },
}

#[derive(StructOpt)]
pub enum I3MProject {
    #[structopt(about = "Create a new project (new)")]
    New {
        #[structopt(short, long)]
        name: String,
    },

    #[structopt(about = "Build a project (build)")]
    Build {
        #[structopt(short, long)]
        proj: String,
    },

    #[structopt(about = "Run a project (run)")]
    Run {
        #[structopt(short, long)]
        proj: String,
    },

    #[structopt(about = "Initialize I3M engine core (core)")]
    Core,
}

pub async fn handle_registry_command(cmd: Registry) -> Result<()> {
    match cmd {
        Registry::Add { module, version, deps } => {
            registry::add_module(module, version, deps).await?;
        }
        Registry::List { module } => {
            registry::list_versions(module).await?;
        }
        Registry::Install { module, version } => {
            registry::install_module(module, version).await?;
        }
        Registry::Up { module } => {
            registry::update_module(module).await?;
        }
    }
    Ok(())
}

pub fn handle_project_command(cmd: I3MProject) -> Result<()> {
    match cmd {
        I3MProject::New { name } => i3m_project::create(name)?,
        I3MProject::Build { proj } => i3m_project::build(proj)?,
        I3MProject::Run { proj } => i3m_project::run(proj)?,
        I3MProject::Core => i3m_project::initialize_core()?,
    }
    Ok(())
}
