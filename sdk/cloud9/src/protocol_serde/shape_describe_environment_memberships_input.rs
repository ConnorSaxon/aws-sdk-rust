// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_environment_memberships_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeEnvironmentMembershipsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.user_arn {
        object.key("userArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.environment_id {
        object.key("environmentId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.permissions {
        let mut array_4 = object.key("permissions").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.next_token {
        object.key("nextToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    Ok(())
}

