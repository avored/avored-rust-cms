use juniper::graphql_object;
use crate::avored_state::AvoRedState;
use crate::query::AvoRedQuery;

#[graphql_object]
#[graphql(context = AvoRedState)]
impl AvoRedQuery {
    pub async fn api_version(context: &AvoRedState) -> &'static str   {
        // let current_page: i64= 0;
        // let order = String::from("");
        // let admin_user_pagination = context
        //     .admin_user_service
        //     .test(&context.db, current_page, order).await.unwrap();
        // println!("ctx : {:?}", context);
        //
        "1.0"
        // Ok(admin_user_pagination.as_str())
    }
}