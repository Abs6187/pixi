use crate::cli::project::system_requirements::SystemRequirementEnum;
use crate::Project;
use clap::Parser;
use pixi_manifest::{FeatureName, SystemRequirements};

#[derive(Parser, Debug)]
pub struct Args {
    /// The name of the system requirement to remove.
    pub requirement: SystemRequirementEnum,

    /// The name of the feature to modify.
    #[clap(long, short)]
    pub feature: Option<String>,

    /// The render layers to use
    #[clap(long)]
    pub render_layers: Option<String>,
}

pub async fn execute(mut project: Project, args: Args) -> miette::Result<()> {
    let feature_name = args
        .feature
        .clone()
        .map_or(FeatureName::Default, FeatureName::Named);

    // Remove the system requirement from the manifest
    project
        .manifest
        .remove_system_requirement(args.requirement, &feature_name)?;

    if let Some(render_layers) = args.render_layers {
        // Handle render layers argument
        // Add your logic here
    }

    // Save the project to disk
    project.save()?;

    Ok(())
}
