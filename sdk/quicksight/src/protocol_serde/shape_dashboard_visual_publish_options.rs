// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_dashboard_visual_publish_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DashboardVisualPublishOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.export_hidden_fields_option {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ExportHiddenFieldsOption").start_object();
        crate::protocol_serde::shape_export_hidden_fields_option::ser_export_hidden_fields_option(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

