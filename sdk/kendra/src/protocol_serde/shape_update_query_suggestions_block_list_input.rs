// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_query_suggestions_block_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateQuerySuggestionsBlockListInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.index_id {
        object.key("IndexId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.id {
        object.key("Id").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.source_s3_path {
        #[allow(unused_mut)]
        let mut object_6 = object.key("SourceS3Path").start_object();
        crate::protocol_serde::shape_s3_path::ser_s3_path(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.role_arn {
        object.key("RoleArn").string(var_7.as_str());
    }
    Ok(())
}

