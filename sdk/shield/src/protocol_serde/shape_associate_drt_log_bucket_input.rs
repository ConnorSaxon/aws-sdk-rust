// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_drt_log_bucket_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateDrtLogBucketInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.log_bucket {
        object.key("LogBucket").string(var_1.as_str());
    }
    Ok(())
}

