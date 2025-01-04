use crate::{
    client::{Licheszter, UrlBase},
    error::Result,
    models::{analysis::CloudAnalysis, game::VariantMode},
};

impl Licheszter {
    pub async fn analysis_cloud(
        &self,
        fen: &str,
        multi_pv: Option<u8>,
        variant: Option<VariantMode>,
    ) -> Result<CloudAnalysis> {
        let url = self.request_url(UrlBase::Lichess, "api/cloud-eval");
        let mut builder = self
            .client
            .get(url)
            .query(&[("fen", fen.replace(" ", "_"))]);

        // Add the multiPv amount as a query parameter if it's present
        if let Some(multi_pv) = multi_pv {
            builder = builder.query(&[("multiPv", multi_pv)]);
        }

        // Add the variant as a query parameter if it's present
        if let Some(variant) = variant {
            builder = builder.query(&[("variant", variant)]);
        }

        self.into::<CloudAnalysis>(builder).await
    }
}
