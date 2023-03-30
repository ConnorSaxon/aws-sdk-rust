// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_pipe_target_http_parameters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PipeTargetHttpParameters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.path_parameter_values {
        let mut array_2 = object.key("PathParameterValues").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.header_parameters {
        #[allow(unused_mut)]
        let mut object_5 = object.key("HeaderParameters").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.query_string_parameters {
        #[allow(unused_mut)]
        let mut object_9 = object.key("QueryStringParameters").start_object();
        for (key_10, value_11) in var_8 {
             {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}

pub(crate) fn de_pipe_target_http_parameters<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PipeTargetHttpParameters>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::pipe_target_http_parameters::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "PathParameterValues" => {
                                builder = builder.set_path_parameter_values(
                                    crate::protocol_serde::shape_path_parameter_list::de_path_parameter_list(tokens)?
                                );
                            }
                            "HeaderParameters" => {
                                builder = builder.set_header_parameters(
                                    crate::protocol_serde::shape_header_parameters_map::de_header_parameters_map(tokens)?
                                );
                            }
                            "QueryStringParameters" => {
                                builder = builder.set_query_string_parameters(
                                    crate::protocol_serde::shape_query_string_parameters_map::de_query_string_parameters_map(tokens)?
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

