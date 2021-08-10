use crate::{handlers::Handler, ImportCommand};

impl Handler for ImportCommand {
    fn handle(&self, client: &rufm_core::Client) -> Result<(), Box<dyn std::error::Error>> {
        let ImportCommand::FireflyIii { export_file } = self;

        let file = std::fs::File::open(&export_file)?;
        rufm_import_firefly_iii::import_firefly_iii(client, &file)?;

        Ok(())
    }
}
