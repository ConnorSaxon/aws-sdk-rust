// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sidewalk_update_account(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SidewalkUpdateAccount) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.app_server_private_key {
        object.key("AppServerPrivateKey").string(var_1.as_str());
    }
    Ok(())
}

