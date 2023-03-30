// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_license_conversion_task_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLicenseConversionTaskForResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_arn {
        object.key("ResourceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.source_license_context {
        #[allow(unused_mut)]
        let mut object_3 = object.key("SourceLicenseContext").start_object();
        crate::protocol_serde::shape_license_conversion_context::ser_license_conversion_context(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.destination_license_context {
        #[allow(unused_mut)]
        let mut object_5 = object.key("DestinationLicenseContext").start_object();
        crate::protocol_serde::shape_license_conversion_context::ser_license_conversion_context(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

