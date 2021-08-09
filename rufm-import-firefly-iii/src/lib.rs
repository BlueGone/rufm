extern crate rufm_core;
use rufm_core::Client;

#[derive(Debug)]
pub struct ImportFireflyIiiError;

pub fn import_firefly_iii<R: std::io::Read>(
    _client: &Client,
    _rdr: R,
) -> Result<(), ImportFireflyIiiError> {
    todo!()
}
