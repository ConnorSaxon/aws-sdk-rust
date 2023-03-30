// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_data_repository_association_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateDataRepositoryAssociationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.association_id {
        object.key("AssociationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.imported_file_chunk_size {
        object.key("ImportedFileChunkSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.s3 {
        #[allow(unused_mut)]
        let mut object_5 = object.key("S3").start_object();
        crate::protocol_serde::shape_s3_data_repository_configuration::ser_s3_data_repository_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

