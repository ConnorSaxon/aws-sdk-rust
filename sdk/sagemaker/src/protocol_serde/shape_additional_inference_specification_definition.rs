// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_additional_inference_specification_definition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AdditionalInferenceSpecificationDefinition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.containers {
        let mut array_4 = object.key("Containers").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_model_package_container_definition::ser_model_package_container_definition(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.supported_transform_instance_types {
        let mut array_8 = object.key("SupportedTransformInstanceTypes").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.supported_realtime_inference_instance_types {
        let mut array_11 = object.key("SupportedRealtimeInferenceInstanceTypes").start_array();
        for item_12 in var_10 {
             {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.supported_content_types {
        let mut array_14 = object.key("SupportedContentTypes").start_array();
        for item_15 in var_13 {
             {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.supported_response_mime_types {
        let mut array_17 = object.key("SupportedResponseMIMETypes").start_array();
        for item_18 in var_16 {
             {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    Ok(())
}

pub(crate) fn de_additional_inference_specification_definition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AdditionalInferenceSpecificationDefinition>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::additional_inference_specification_definition::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Containers" => {
                                builder = builder.set_containers(
                                    crate::protocol_serde::shape_model_package_container_definition_list::de_model_package_container_definition_list(tokens)?
                                );
                            }
                            "SupportedTransformInstanceTypes" => {
                                builder = builder.set_supported_transform_instance_types(
                                    crate::protocol_serde::shape_transform_instance_types::de_transform_instance_types(tokens)?
                                );
                            }
                            "SupportedRealtimeInferenceInstanceTypes" => {
                                builder = builder.set_supported_realtime_inference_instance_types(
                                    crate::protocol_serde::shape_realtime_inference_instance_types::de_realtime_inference_instance_types(tokens)?
                                );
                            }
                            "SupportedContentTypes" => {
                                builder = builder.set_supported_content_types(
                                    crate::protocol_serde::shape_content_types::de_content_types(tokens)?
                                );
                            }
                            "SupportedResponseMIMETypes" => {
                                builder = builder.set_supported_response_mime_types(
                                    crate::protocol_serde::shape_response_mime_types::de_response_mime_types(tokens)?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

