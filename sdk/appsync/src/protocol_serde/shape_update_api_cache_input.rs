// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_api_cache_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateApiCacheInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.api_caching_behavior {
        object.key("apiCachingBehavior").string(var_1.as_str());
    }
     {
        object.key("ttl").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.ttl).into()));
    }
    if let Some(var_2) = &input.r#type {
        object.key("type").string(var_2.as_str());
    }
    Ok(())
}

