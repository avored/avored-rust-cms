use crate::avored_state::AvoRedState;
use crate::query::AvoRedQuery;
use juniper::{
    graphql_object, FieldResult, GraphQLObject, GraphQLScalar, InputValue, ParseScalarResult,
    ParseScalarValue, ScalarToken, ScalarValue, Value,
};
use surrealdb::sql::Datetime;

#[graphql_object]
#[graphql(context = AvoRedState)]
impl AvoRedQuery {
    pub async fn api_version(context: &AvoRedState) -> FieldResult<TestRoleModel> {
        let mut test = TestRoleModel::default();
        let user = UserId(Datetime::default());
        test.id = user;
        let _test_state = context;
        // let current_page: i64= 0;
        // let order = String::from("");
        // let admin_user_pagination = context
        //     .admin_user_service
        //     .test(&context.db, current_page, order).await.unwrap();
        // println!("ctx : {:?}", context);
        //
        // "1.0"
        Ok(test)
        // Ok(admin_user_pagination.as_str())
    }
}

#[derive(Debug, Default, GraphQLObject)]
pub struct TestRoleModel {
    pub id: UserId,
    pub name: String,
    pub created: String,
}

#[derive(GraphQLScalar, Debug, Default)]
#[graphql(
    to_output_with = to_output,
    parse_token_with = parse_token,
    from_input_with = from_input,
    transparent
)]
pub struct UserId(Datetime);

fn to_output<S: ScalarValue>(v: &UserId) -> Value<S> {
    println!("{:?}", v);
    let test = &v.0;
    // let inc = v.0 + 1;
    Value::from(test.to_string())
}

fn parse_token<S: ScalarValue>(value: ScalarToken<'_>) -> ParseScalarResult<S> {
    <String as ParseScalarValue<S>>::from_str(value)
        .or_else(|_| <i32 as ParseScalarValue<S>>::from_str(value))
}

fn from_input<S>(_input: &InputValue<S>) -> Result<UserId, String>
where
    S: ScalarValue,
{
    // Datetime::default()
    // input.as_string_value()
    //     .and_then(|s| {
    //         Datetime::try_from(s).map_err(|_e| format!("invalid date")) })
    //
    Ok(UserId(Datetime::default()))
    // input.as_string_value()
    //     .ok_or_else(|| format!("Expected `String`, found: {input}"))
    //     .and_then(|s| {
    //         let date = Datetime::try_from(s);
    //         // UserId().map_err(|_e| format!("invalid date"))
    //     })
}
