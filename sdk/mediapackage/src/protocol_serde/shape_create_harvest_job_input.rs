// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_harvest_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateHarvestJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.end_time {
        object.key("endTime").string(var_1.as_str());
    }
    if let Some(var_2) = &input.id {
        object.key("id").string(var_2.as_str());
    }
    if let Some(var_3) = &input.origin_endpoint_id {
        object.key("originEndpointId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.s3_destination {
        #[allow(unused_mut)]
        let mut object_5 = object.key("s3Destination").start_object();
        crate::protocol_serde::shape_s3_destination::ser_s3_destination(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.start_time {
        object.key("startTime").string(var_6.as_str());
    }
    Ok(())
}

