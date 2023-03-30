// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_inference_specification(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InferenceSpecification) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.containers {
        let mut array_2 = object.key("Containers").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_model_package_container_definition::ser_model_package_container_definition(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.supported_transform_instance_types {
        let mut array_6 = object.key("SupportedTransformInstanceTypes").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.supported_realtime_inference_instance_types {
        let mut array_9 = object.key("SupportedRealtimeInferenceInstanceTypes").start_array();
        for item_10 in var_8 {
             {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.supported_content_types {
        let mut array_12 = object.key("SupportedContentTypes").start_array();
        for item_13 in var_11 {
             {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.supported_response_mime_types {
        let mut array_15 = object.key("SupportedResponseMIMETypes").start_array();
        for item_16 in var_14 {
             {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    Ok(())
}

pub(crate) fn de_inference_specification<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::InferenceSpecification>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::inference_specification::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
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

