// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_grpc_retry_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::GrpcRetryPolicy) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.per_retry_timeout {
        #[allow(unused_mut)]
        let mut object_2 = object.key("perRetryTimeout").start_object();
        crate::protocol_serde::shape_duration::ser_duration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.max_retries {
        object.key("maxRetries").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.http_retry_events {
        let mut array_5 = object.key("httpRetryEvents").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.tcp_retry_events {
        let mut array_8 = object.key("tcpRetryEvents").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.grpc_retry_events {
        let mut array_11 = object.key("grpcRetryEvents").start_array();
        for item_12 in var_10 {
             {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    Ok(())
}

pub(crate) fn de_grpc_retry_policy<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::GrpcRetryPolicy>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::grpc_retry_policy::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "perRetryTimeout" => {
                                builder = builder.set_per_retry_timeout(
                                    crate::protocol_serde::shape_duration::de_duration(tokens)?
                                );
                            }
                            "maxRetries" => {
                                builder = builder.set_max_retries(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
                                );
                            }
                            "httpRetryEvents" => {
                                builder = builder.set_http_retry_events(
                                    crate::protocol_serde::shape_http_retry_policy_events::de_http_retry_policy_events(tokens)?
                                );
                            }
                            "tcpRetryEvents" => {
                                builder = builder.set_tcp_retry_events(
                                    crate::protocol_serde::shape_tcp_retry_policy_events::de_tcp_retry_policy_events(tokens)?
                                );
                            }
                            "grpcRetryEvents" => {
                                builder = builder.set_grpc_retry_events(
                                    crate::protocol_serde::shape_grpc_retry_policy_events::de_grpc_retry_policy_events(tokens)?
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

