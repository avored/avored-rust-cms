use handlebars::Handlebars;

use crate::providers::avored_view_provider::AvoRedViewProvider;

pub struct AvoRedState {
    pub handlebars: Handlebars<'static>,
}

impl AvoRedState {
    pub async fn new() -> AvoRedState {
        let avored_view_provider = AvoRedViewProvider::register();
        
        AvoRedState {
            handlebars: avored_view_provider.handlebars,

        }
    }
}
