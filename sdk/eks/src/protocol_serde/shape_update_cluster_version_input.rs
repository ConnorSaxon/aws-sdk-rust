// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_cluster_version_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateClusterVersionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("clientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.version {
        object.key("version").string(var_2.as_str());
    }
    Ok(())
}

