// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_query_suggestions_block_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateQuerySuggestionsBlockListInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.index_id {
        object.key("IndexId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.source_s3_path {
        #[allow(unused_mut)]
        let mut object_5 = object.key("SourceS3Path").start_object();
        crate::protocol_serde::shape_s3_path::ser_s3_path(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.client_token {
        object.key("ClientToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.role_arn {
        object.key("RoleArn").string(var_7.as_str());
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

