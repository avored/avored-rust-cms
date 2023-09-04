use handlebars::Handlebars;
use crate::error::Result;
use crate::providers::avored_config_provider::AvoRedConfigProvider;
use crate::providers::avored_view_provider::AvoRedViewProvider;

pub struct AvoRedState {
    pub handlebars: Handlebars<'static>,
    pub config: AvoRedConfigProvider,
}

impl AvoRedState {
    pub async fn new() -> Result<AvoRedState> {
        let avored_view_provider = AvoRedViewProvider::register()?;
        let avored_config_provider = AvoRedConfigProvider::register()?;
        
        Ok(AvoRedState {
            handlebars: avored_view_provider.handlebars,
            config: avored_config_provider
        })
    }
}
