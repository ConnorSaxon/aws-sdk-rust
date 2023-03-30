// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_asset_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateAssetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.asset_description {
        object.key("assetDescription").string(var_1.as_str());
    }
    if let Some(var_2) = &input.asset_name {
        object.key("assetName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.client_token {
        object.key("clientToken").string(var_3.as_str());
    }
    Ok(())
}

