// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_poll_for_jobs_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PollForJobsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action_type_id {
        #[allow(unused_mut)]
        let mut object_2 = object.key("actionTypeId").start_object();
        crate::protocol_serde::shape_action_type_id::ser_action_type_id(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.max_batch_size {
        object.key("maxBatchSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.query_param {
        #[allow(unused_mut)]
        let mut object_5 = object.key("queryParam").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    Ok(())
}

