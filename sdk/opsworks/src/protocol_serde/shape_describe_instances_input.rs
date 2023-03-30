// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_instances_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeInstancesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.stack_id {
        object.key("StackId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.layer_id {
        object.key("LayerId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_ids {
        let mut array_4 = object.key("InstanceIds").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}

