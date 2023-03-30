// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_buckets_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetBucketsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.bucket_name {
        object.key("bucketName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.page_token {
        object.key("pageToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.include_connected_resources {
        object.key("includeConnectedResources").boolean(*var_3);
    }
    Ok(())
}

