// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_reset_distribution_cache_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ResetDistributionCacheInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.distribution_name {
        object.key("distributionName").string(var_1.as_str());
    }
    Ok(())
}

