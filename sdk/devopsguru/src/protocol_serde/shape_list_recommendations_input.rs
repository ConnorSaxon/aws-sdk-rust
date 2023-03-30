// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_recommendations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListRecommendationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("AccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.insight_id {
        object.key("InsightId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.locale {
        object.key("Locale").string(var_3.as_str());
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    Ok(())
}

