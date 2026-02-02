use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::{analysis::CloudAnalysis, game::VariantMode},
};

impl Licheszter {
    /// Get cloud evaluation of a position from the Lichess opening explorer.
    ///
    /// # Arguments
    /// * `fen` - FEN notation of the position to analyze
    /// * `multi_pv` - Optional number of principal variations to return (MultiPV)
    /// * `variant` - Optional chess variant (standard, chess960, etc.)
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
