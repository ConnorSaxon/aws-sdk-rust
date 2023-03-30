// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_application_reference_data_source_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddApplicationReferenceDataSourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.application_name {
        object.key("ApplicationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.current_application_version_id {
        object.key("CurrentApplicationVersionId").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.reference_data_source {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ReferenceDataSource").start_object();
        crate::protocol_serde::shape_reference_data_source::ser_reference_data_source(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

