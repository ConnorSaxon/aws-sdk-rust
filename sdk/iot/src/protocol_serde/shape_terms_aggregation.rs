// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_terms_aggregation(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TermsAggregation) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.max_buckets != 0 {
        object.key("maxBuckets").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_buckets).into()));
    }
    Ok(())
}

