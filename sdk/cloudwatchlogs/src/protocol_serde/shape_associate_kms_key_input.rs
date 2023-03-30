// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_kms_key_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateKmsKeyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.log_group_name {
        object.key("logGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_2.as_str());
    }
    Ok(())
}

