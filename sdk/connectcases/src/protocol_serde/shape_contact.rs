// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_contact(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Contact) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.contact_arn {
        object.key("contactArn").string(var_1.as_str());
    }
    Ok(())
}

