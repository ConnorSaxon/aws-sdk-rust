// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_bundles_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetBundlesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.include_inactive {
        object.key("includeInactive").boolean(*var_1);
    }
    if let Some(var_2) = &input.page_token {
        object.key("pageToken").string(var_2.as_str());
    }
    Ok(())
}

