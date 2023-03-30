// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_compute_environment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateComputeEnvironmentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.compute_environment_name {
        object.key("computeEnvironmentName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.compute_resources {
        #[allow(unused_mut)]
        let mut object_3 = object.key("computeResources").start_object();
        crate::protocol_serde::shape_compute_resource::ser_compute_resource(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.eks_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("eksConfiguration").start_object();
        crate::protocol_serde::shape_eks_configuration::ser_eks_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.service_role {
        object.key("serviceRole").string(var_6.as_str());
    }
    if let Some(var_7) = &input.state {
        object.key("state").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        #[allow(unused_mut)]
        let mut object_9 = object.key("tags").start_object();
        for (key_10, value_11) in var_8 {
             {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    if let Some(var_12) = &input.r#type {
        object.key("type").string(var_12.as_str());
    }
    if let Some(var_13) = &input.unmanagedv_cpus {
        object.key("unmanagedvCpus").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_13).into()));
    }
    Ok(())
}

