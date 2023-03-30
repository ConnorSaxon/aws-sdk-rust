// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lookup_developer_identity_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::LookupDeveloperIdentityInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identity_id {
        object.key("IdentityId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.developer_user_identifier {
        object.key("DeveloperUserIdentifier").string(var_3.as_str());
    }
    if input.max_results != 0 {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    Ok(())
}

