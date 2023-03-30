// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_script_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateScriptInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.version {
        object.key("Version").string(var_2.as_str());
    }
    if let Some(var_3) = &input.storage_location {
        #[allow(unused_mut)]
        let mut object_4 = object.key("StorageLocation").start_object();
        crate::protocol_serde::shape_s3_location::ser_s3_location(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.zip_file {
        object.key("ZipFile").string_unchecked(&aws_smithy_types::base64::encode(var_5));
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

