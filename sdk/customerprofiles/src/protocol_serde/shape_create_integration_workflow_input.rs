// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_integration_workflow_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateIntegrationWorkflowInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.integration_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("IntegrationConfig").start_object();
        crate::protocol_serde::shape_integration_config::ser_integration_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.object_type_name {
        object.key("ObjectTypeName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.role_arn {
        object.key("RoleArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Tags").start_object();
        for (key_7, value_8) in var_5 {
             {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    if let Some(var_9) = &input.workflow_type {
        object.key("WorkflowType").string(var_9.as_str());
    }
    Ok(())
}

