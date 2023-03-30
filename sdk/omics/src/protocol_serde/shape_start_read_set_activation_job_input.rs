// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_read_set_activation_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartReadSetActivationJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sources {
        let mut array_3 = object.key("sources").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_start_read_set_activation_job_source_item::ser_start_read_set_activation_job_source_item(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

