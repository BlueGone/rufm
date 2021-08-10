use crate::ImportCommand;

pub fn handle_import_command(
    client: &rufm_core::Client,
    import_command: ImportCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    let ImportCommand::FireflyIii { export_file } = import_command;

    let file = std::fs::File::open(&export_file)?;
    rufm_import_firefly_iii::import_firefly_iii(client, &file)?;

    Ok(())
}
