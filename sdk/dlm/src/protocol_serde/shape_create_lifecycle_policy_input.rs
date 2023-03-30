// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_lifecycle_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLifecyclePolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.policy_details {
        #[allow(unused_mut)]
        let mut object_4 = object.key("PolicyDetails").start_object();
        crate::protocol_serde::shape_policy_details::ser_policy_details(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.state {
        object.key("State").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        #[allow(unused_mut)]
        let mut object_7 = object.key("Tags").start_object();
        for (key_8, value_9) in var_6 {
             {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    Ok(())
}

