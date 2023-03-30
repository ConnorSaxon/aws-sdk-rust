// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_recording_group(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RecordingGroup) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.all_supported {
        object.key("allSupported").boolean(input.all_supported);
    }
    if input.include_global_resource_types {
        object.key("includeGlobalResourceTypes").boolean(input.include_global_resource_types);
    }
    if let Some(var_1) = &input.resource_types {
        let mut array_2 = object.key("resourceTypes").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub(crate) fn de_recording_group<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RecordingGroup>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::recording_group::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "allSupported" => {
                                builder = builder.set_all_supported(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "includeGlobalResourceTypes" => {
                                builder = builder.set_include_global_resource_types(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "resourceTypes" => {
                                builder = builder.set_resource_types(
                                    crate::protocol_serde::shape_resource_type_list::de_resource_type_list(tokens)?
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

