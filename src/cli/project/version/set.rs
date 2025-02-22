use crate::Project;
use clap::Parser;

#[derive(Parser, Debug, Default)]
pub struct Args {
    /// The new project version
    #[clap(required = true, num_args = 1)]
    pub version: String,

    /// The render layers to use
    #[clap(long)]
    pub render_layers: Option<String>,
}

pub async fn execute(mut project: Project, args: Args) -> miette::Result<()> {
    // Set the version
    project.manifest.set_version(&args.version)?;

    // Save the manifest on disk
    project.save()?;

    // Report back to the user
    eprintln!(
        "{}Updated project version to '{}'.",
        console::style(console::Emoji("✔ ", "")).green(),
        project
            .version()
            .as_ref()
            .expect("we just set the version, so it should be there")
    );

    if let Some(layers) = args.render_layers {
        eprintln!(
            "Render layers applied: {}",
            console::style(layers).bold()
        );
    }

    Ok(())
}
