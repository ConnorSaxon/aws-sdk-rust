// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_thesaurus_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateThesaurusInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.index_id {
        object.key("IndexId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.role_arn {
        object.key("RoleArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.source_s3_path {
        #[allow(unused_mut)]
        let mut object_10 = object.key("SourceS3Path").start_object();
        crate::protocol_serde::shape_s3_path::ser_s3_path(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.client_token {
        object.key("ClientToken").string(var_11.as_str());
    }
    Ok(())
}

