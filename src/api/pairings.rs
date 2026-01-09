use reqwest::header;

use crate::{
    client::{Licheszter, UrlBase},
    config::pairings::BulkPairingOptions,
    error::Result,
    models::pairings::{BulkPairing, BulkPairings},
};

impl Licheszter {
    /// Get a list of bulk pairings you created.
    pub async fn bulk_pairings_list(&self) -> Result<Vec<BulkPairing>> {
        let url = self.req_url(UrlBase::Lichess, "api/bulk-pairing");
        let builder = self.client.get(url);

        Ok(self.into::<BulkPairings>(builder).await?.bulks)
    }

    /// Schedule many games at once, up to 24 hours in advance.
    /// Authentication tokens are required for all paired players.
    /// You can schedule up to 500 games every 10 minutes.
    /// If games have a real-time clock, each player must have only one pairing.
    /// For correspondence games, players can have multiple pairings within the same bulk.
    /// The pairing must contain time control and player information.
    pub async fn bulk_pairings_create(&self, options: &BulkPairingOptions) -> Result<BulkPairing> {
        let url = self.req_url(UrlBase::Lichess, "api/bulk-pairing");
        let mut builder = self.client.post(url);

        // Add the options to the request
        let encoded = comma_serde_urlencoded::to_string(options)?;
        builder = builder
            .body(encoded)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");

        self.into::<BulkPairing>(builder).await
    }
}
