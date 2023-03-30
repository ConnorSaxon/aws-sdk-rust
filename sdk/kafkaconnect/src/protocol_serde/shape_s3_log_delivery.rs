// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_log_delivery(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3LogDelivery) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.bucket {
        object.key("bucket").string(var_1.as_str());
    }
     {
        object.key("enabled").boolean(input.enabled);
    }
    if let Some(var_2) = &input.prefix {
        object.key("prefix").string(var_2.as_str());
    }
    Ok(())
}

