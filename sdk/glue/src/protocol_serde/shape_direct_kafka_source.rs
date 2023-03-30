// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_direct_kafka_source(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DirectKafkaSource) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.streaming_options {
        #[allow(unused_mut)]
        let mut object_3 = object.key("StreamingOptions").start_object();
        crate::protocol_serde::shape_kafka_streaming_source_options::ser_kafka_streaming_source_options(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.window_size {
        object.key("WindowSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    if let Some(var_5) = &input.detect_schema {
        object.key("DetectSchema").boolean(*var_5);
    }
    if let Some(var_6) = &input.data_preview_options {
        #[allow(unused_mut)]
        let mut object_7 = object.key("DataPreviewOptions").start_object();
        crate::protocol_serde::shape_streaming_data_preview_options::ser_streaming_data_preview_options(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}

pub(crate) fn de_direct_kafka_source<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DirectKafkaSource>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::direct_kafka_source::Builder::default();
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
                            "StreamingOptions" => {
                                builder = builder.set_streaming_options(
                                    crate::protocol_serde::shape_kafka_streaming_source_options::de_kafka_streaming_source_options(tokens)?
                                );
                            }
                            "WindowSize" => {
                                builder = builder.set_window_size(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "DetectSchema" => {
                                builder = builder.set_detect_schema(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "DataPreviewOptions" => {
                                builder = builder.set_data_preview_options(
                                    crate::protocol_serde::shape_streaming_data_preview_options::de_streaming_data_preview_options(tokens)?
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

