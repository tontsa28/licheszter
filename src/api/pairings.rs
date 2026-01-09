use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::pairings::{BulkPairing, BulkPairings},
};

impl Licheszter {
    pub async fn bulk_pairings_list(&self) -> Result<Vec<BulkPairing>> {
        let url = self.req_url(UrlBase::Lichess, "api/bulk-pairing");
        let builder = self.client.get(url);

        Ok(self.into::<BulkPairings>(builder).await?.bulks)
    }
}
