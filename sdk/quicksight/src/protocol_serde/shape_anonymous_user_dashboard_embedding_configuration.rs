// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_anonymous_user_dashboard_embedding_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AnonymousUserDashboardEmbeddingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.initial_dashboard_id {
        object.key("InitialDashboardId").string(var_1.as_str());
    }
    Ok(())
}

