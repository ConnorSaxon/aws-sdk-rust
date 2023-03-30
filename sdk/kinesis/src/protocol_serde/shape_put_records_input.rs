// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_records_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutRecordsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.records {
        let mut array_2 = object.key("Records").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_put_records_request_entry::ser_put_records_request_entry(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.stream_name {
        object.key("StreamName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.stream_arn {
        object.key("StreamARN").string(var_6.as_str());
    }
    Ok(())
}

