// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_anonymous_user_dashboard_visual_embedding_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AnonymousUserDashboardVisualEmbeddingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.initial_dashboard_visual_id {
        #[allow(unused_mut)]
        let mut object_2 = object.key("InitialDashboardVisualId").start_object();
        crate::protocol_serde::shape_dashboard_visual_id::ser_dashboard_visual_id(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

