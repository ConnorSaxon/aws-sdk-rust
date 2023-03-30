// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_annotation_store_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAnnotationStoreInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.reference {
        #[allow(unused_mut)]
        let mut object_4 = object.key("reference").start_object();
        crate::protocol_serde::shape_reference_item::ser_reference_item(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.sse_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("sseConfig").start_object();
        crate::protocol_serde::shape_sse_config::ser_sse_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.store_format {
        object.key("storeFormat").string(var_7.as_str());
    }
    if let Some(var_8) = &input.store_options {
        #[allow(unused_mut)]
        let mut object_9 = object.key("storeOptions").start_object();
        crate::protocol_serde::shape_store_options::ser_store_options(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.tags {
        #[allow(unused_mut)]
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
             {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

