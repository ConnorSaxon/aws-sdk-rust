// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_host_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateHostInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.host_arn {
        object.key("HostArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.provider_endpoint {
        object.key("ProviderEndpoint").string(var_2.as_str());
    }
    if let Some(var_3) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("VpcConfiguration").start_object();
        crate::protocol_serde::shape_vpc_configuration::ser_vpc_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

