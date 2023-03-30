// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_availability_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateAvailabilityConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.domain_name {
        object.key("DomainName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ews_provider {
        #[allow(unused_mut)]
        let mut object_4 = object.key("EwsProvider").start_object();
        crate::protocol_serde::shape_ews_availability_provider::ser_ews_availability_provider(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.lambda_provider {
        #[allow(unused_mut)]
        let mut object_6 = object.key("LambdaProvider").start_object();
        crate::protocol_serde::shape_lambda_availability_provider::ser_lambda_availability_provider(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

