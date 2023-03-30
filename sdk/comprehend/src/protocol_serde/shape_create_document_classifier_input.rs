// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_document_classifier_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDocumentClassifierInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.document_classifier_name {
        object.key("DocumentClassifierName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.version_name {
        object.key("VersionName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.data_access_role_arn {
        object.key("DataAccessRoleArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("Tags").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.input_data_config {
        #[allow(unused_mut)]
        let mut object_9 = object.key("InputDataConfig").start_object();
        crate::protocol_serde::shape_document_classifier_input_data_config::ser_document_classifier_input_data_config(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.output_data_config {
        #[allow(unused_mut)]
        let mut object_11 = object.key("OutputDataConfig").start_object();
        crate::protocol_serde::shape_document_classifier_output_data_config::ser_document_classifier_output_data_config(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.language_code {
        object.key("LanguageCode").string(var_13.as_str());
    }
    if let Some(var_14) = &input.volume_kms_key_id {
        object.key("VolumeKmsKeyId").string(var_14.as_str());
    }
    if let Some(var_15) = &input.vpc_config {
        #[allow(unused_mut)]
        let mut object_16 = object.key("VpcConfig").start_object();
        crate::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.mode {
        object.key("Mode").string(var_17.as_str());
    }
    if let Some(var_18) = &input.model_kms_key_id {
        object.key("ModelKmsKeyId").string(var_18.as_str());
    }
    if let Some(var_19) = &input.model_policy {
        object.key("ModelPolicy").string(var_19.as_str());
    }
    Ok(())
}

