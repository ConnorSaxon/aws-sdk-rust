// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_raw_email(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RawEmail) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.data {
        object.key("Data").string_unchecked(&aws_smithy_types::base64::encode(var_1));
    }
    Ok(())
}

