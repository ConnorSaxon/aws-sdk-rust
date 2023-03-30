// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_api_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateApiInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.api_key_selection_expression {
        object.key("apiKeySelectionExpression").string(var_1.as_str());
    }
    if let Some(var_2) = &input.cors_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("corsConfiguration").start_object();
        crate::protocol_serde::shape_cors::ser_cors(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.credentials_arn {
        object.key("credentialsArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.description {
        object.key("description").string(var_5.as_str());
    }
    if input.disable_execute_api_endpoint {
        object.key("disableExecuteApiEndpoint").boolean(input.disable_execute_api_endpoint);
    }
    if input.disable_schema_validation {
        object.key("disableSchemaValidation").boolean(input.disable_schema_validation);
    }
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6.as_str());
    }
    if let Some(var_7) = &input.route_key {
        object.key("routeKey").string(var_7.as_str());
    }
    if let Some(var_8) = &input.route_selection_expression {
        object.key("routeSelectionExpression").string(var_8.as_str());
    }
    if let Some(var_9) = &input.target {
        object.key("target").string(var_9.as_str());
    }
    if let Some(var_10) = &input.version {
        object.key("version").string(var_10.as_str());
    }
    Ok(())
}

