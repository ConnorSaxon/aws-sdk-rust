// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_elastic_ips_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeElasticIpsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.instance_id {
        object.key("InstanceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.stack_id {
        object.key("StackId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ips {
        let mut array_4 = object.key("Ips").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}

