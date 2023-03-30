// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_telemetry_records_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutTelemetryRecordsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ec2_instance_id {
        object.key("EC2InstanceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.hostname {
        object.key("Hostname").string(var_2.as_str());
    }
    if let Some(var_3) = &input.resource_arn {
        object.key("ResourceARN").string(var_3.as_str());
    }
    if let Some(var_4) = &input.telemetry_records {
        let mut array_5 = object.key("TelemetryRecords").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_telemetry_record::ser_telemetry_record(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}

