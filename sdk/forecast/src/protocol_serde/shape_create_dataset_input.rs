// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_dataset_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDatasetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.dataset_name {
        object.key("DatasetName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.domain {
        object.key("Domain").string(var_2.as_str());
    }
    if let Some(var_3) = &input.dataset_type {
        object.key("DatasetType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.data_frequency {
        object.key("DataFrequency").string(var_4.as_str());
    }
    if let Some(var_5) = &input.schema {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Schema").start_object();
        crate::protocol_serde::shape_schema::ser_schema(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.encryption_config {
        #[allow(unused_mut)]
        let mut object_8 = object.key("EncryptionConfig").start_object();
        crate::protocol_serde::shape_encryption_config::ser_encryption_config(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.tags {
        let mut array_10 = object.key("Tags").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    Ok(())
}

