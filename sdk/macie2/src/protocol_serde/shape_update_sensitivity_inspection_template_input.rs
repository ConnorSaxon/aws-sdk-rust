// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_sensitivity_inspection_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSensitivityInspectionTemplateInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.excludes {
        #[allow(unused_mut)]
        let mut object_3 = object.key("excludes").start_object();
        crate::protocol_serde::shape_sensitivity_inspection_template_excludes::ser_sensitivity_inspection_template_excludes(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.includes {
        #[allow(unused_mut)]
        let mut object_5 = object.key("includes").start_object();
        crate::protocol_serde::shape_sensitivity_inspection_template_includes::ser_sensitivity_inspection_template_includes(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

