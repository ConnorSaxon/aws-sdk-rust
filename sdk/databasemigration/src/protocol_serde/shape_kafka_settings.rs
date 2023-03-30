// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_kafka_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::KafkaSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.broker {
        object.key("Broker").string(var_1.as_str());
    }
    if let Some(var_2) = &input.topic {
        object.key("Topic").string(var_2.as_str());
    }
    if let Some(var_3) = &input.message_format {
        object.key("MessageFormat").string(var_3.as_str());
    }
    if let Some(var_4) = &input.include_transaction_details {
        object.key("IncludeTransactionDetails").boolean(*var_4);
    }
    if let Some(var_5) = &input.include_partition_value {
        object.key("IncludePartitionValue").boolean(*var_5);
    }
    if let Some(var_6) = &input.partition_include_schema_table {
        object.key("PartitionIncludeSchemaTable").boolean(*var_6);
    }
    if let Some(var_7) = &input.include_table_alter_operations {
        object.key("IncludeTableAlterOperations").boolean(*var_7);
    }
    if let Some(var_8) = &input.include_control_details {
        object.key("IncludeControlDetails").boolean(*var_8);
    }
    if let Some(var_9) = &input.message_max_bytes {
        object.key("MessageMaxBytes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_9).into()));
    }
    if let Some(var_10) = &input.include_null_and_empty {
        object.key("IncludeNullAndEmpty").boolean(*var_10);
    }
    if let Some(var_11) = &input.security_protocol {
        object.key("SecurityProtocol").string(var_11.as_str());
    }
    if let Some(var_12) = &input.ssl_client_certificate_arn {
        object.key("SslClientCertificateArn").string(var_12.as_str());
    }
    if let Some(var_13) = &input.ssl_client_key_arn {
        object.key("SslClientKeyArn").string(var_13.as_str());
    }
    if let Some(var_14) = &input.ssl_client_key_password {
        object.key("SslClientKeyPassword").string(var_14.as_str());
    }
    if let Some(var_15) = &input.ssl_ca_certificate_arn {
        object.key("SslCaCertificateArn").string(var_15.as_str());
    }
    if let Some(var_16) = &input.sasl_username {
        object.key("SaslUsername").string(var_16.as_str());
    }
    if let Some(var_17) = &input.sasl_password {
        object.key("SaslPassword").string(var_17.as_str());
    }
    if let Some(var_18) = &input.no_hex_prefix {
        object.key("NoHexPrefix").boolean(*var_18);
    }
    Ok(())
}

pub(crate) fn de_kafka_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::KafkaSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::kafka_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Broker" => {
                                builder = builder.set_broker(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Topic" => {
                                builder = builder.set_topic(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "MessageFormat" => {
                                builder = builder.set_message_format(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::MessageFormatValue::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "IncludeTransactionDetails" => {
                                builder = builder.set_include_transaction_details(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "IncludePartitionValue" => {
                                builder = builder.set_include_partition_value(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "PartitionIncludeSchemaTable" => {
                                builder = builder.set_partition_include_schema_table(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "IncludeTableAlterOperations" => {
                                builder = builder.set_include_table_alter_operations(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "IncludeControlDetails" => {
                                builder = builder.set_include_control_details(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "MessageMaxBytes" => {
                                builder = builder.set_message_max_bytes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "IncludeNullAndEmpty" => {
                                builder = builder.set_include_null_and_empty(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "SecurityProtocol" => {
                                builder = builder.set_security_protocol(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::KafkaSecurityProtocol::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "SslClientCertificateArn" => {
                                builder = builder.set_ssl_client_certificate_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SslClientKeyArn" => {
                                builder = builder.set_ssl_client_key_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SslClientKeyPassword" => {
                                builder = builder.set_ssl_client_key_password(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SslCaCertificateArn" => {
                                builder = builder.set_ssl_ca_certificate_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SaslUsername" => {
                                builder = builder.set_sasl_username(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SaslPassword" => {
                                builder = builder.set_sasl_password(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "NoHexPrefix" => {
                                builder = builder.set_no_hex_prefix(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

