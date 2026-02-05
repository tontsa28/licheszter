use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::{analysis::CloudAnalysis, game::VariantMode},
};

impl Licheszter {
    /// Get the cached evaluation of a position, if available.
    /// Opening positions have higher chances of being available.
    /// There are about 320 million positions in the database.
    /// Up to 5 variations may be available.
    /// Variants are supported.
    /// If you want to download a lot of positions, get the full list from [Lichess database](https://database.lichess.org).
    ///
    /// # Errors
    /// Returns an error if the API request fails or the response cannot be deserialized.
    pub async fn analysis_cloud(
        &self,
        fen: &str,
        multi_pv: Option<u8>,
        variant: Option<VariantMode>,
    ) -> Result<CloudAnalysis> {
        let url = self.req_url(UrlBase::Lichess, "api/cloud-eval");
        let mut builder = self.client.get(url).query(&[("fen", fen.replace(" ", "_"))]);

        // Add the multiPv amount as a query parameter if it's present
        if let Some(multi_pv) = multi_pv {
            builder = builder.query(&[("multiPv", multi_pv)]);
        }

        // Add the variant as a query parameter if it's present
        if let Some(variant) = variant {
            builder = builder.query(&[("variant", variant)]);
        }

        self.to_model::<CloudAnalysis>(builder).await
    }
}
