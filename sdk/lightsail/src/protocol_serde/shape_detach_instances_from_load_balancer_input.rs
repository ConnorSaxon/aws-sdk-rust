// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_detach_instances_from_load_balancer_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DetachInstancesFromLoadBalancerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.load_balancer_name {
        object.key("loadBalancerName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.instance_names {
        let mut array_3 = object.key("instanceNames").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}

