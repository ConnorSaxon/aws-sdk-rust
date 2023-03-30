// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_quantum_task_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateQuantumTaskInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("action").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.device_arn {
        object.key("deviceArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.device_parameters {
        object.key("deviceParameters").string(var_4.as_str());
    }
    if let Some(var_5) = &input.job_token {
        object.key("jobToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.output_s3_bucket {
        object.key("outputS3Bucket").string(var_6.as_str());
    }
    if let Some(var_7) = &input.output_s3_key_prefix {
        object.key("outputS3KeyPrefix").string(var_7.as_str());
    }
    if let Some(var_8) = &input.shots {
        object.key("shots").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
             {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    Ok(())
}

