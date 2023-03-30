// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_fhir_datastore_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateFhirDatastoreInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.datastore_name {
        object.key("DatastoreName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.datastore_type_version {
        object.key("DatastoreTypeVersion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.sse_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SseConfiguration").start_object();
        crate::protocol_serde::shape_sse_configuration::ser_sse_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.preload_data_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("PreloadDataConfig").start_object();
        crate::protocol_serde::shape_preload_data_config::ser_preload_data_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.client_token {
        object.key("ClientToken").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    Ok(())
}

