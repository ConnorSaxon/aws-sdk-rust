// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lambda_device_mount(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LambdaDeviceMount) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.path {
        object.key("path").string(var_1.as_str());
    }
    if let Some(var_2) = &input.permission {
        object.key("permission").string(var_2.as_str());
    }
    if let Some(var_3) = &input.add_group_owner {
        object.key("addGroupOwner").boolean(*var_3);
    }
    Ok(())
}

