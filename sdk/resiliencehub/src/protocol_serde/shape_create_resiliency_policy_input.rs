// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_resiliency_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateResiliencyPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.data_location_constraint {
        object.key("dataLocationConstraint").string(var_2.as_str());
    }
    if let Some(var_3) = &input.policy {
        #[allow(unused_mut)]
        let mut object_4 = object.key("policy").start_object();
        for (key_5, value_6) in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_7 = object_4.key(key_5.as_str()).start_object();
                crate::protocol_serde::shape_failure_policy::ser_failure_policy(&mut object_7, value_6)?;
                object_7.finish();
            }
        }
        object_4.finish();
    }
    if let Some(var_8) = &input.policy_description {
        object.key("policyDescription").string(var_8.as_str());
    }
    if let Some(var_9) = &input.policy_name {
        object.key("policyName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        #[allow(unused_mut)]
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
             {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    if let Some(var_14) = &input.tier {
        object.key("tier").string(var_14.as_str());
    }
    Ok(())
}

