// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_tunnel<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Tunnel>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::tunnel::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "tunnelId" => {
                                builder = builder.set_tunnel_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "tunnelArn" => {
                                builder = builder.set_tunnel_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TunnelStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "sourceConnectionState" => {
                                builder = builder.set_source_connection_state(
                                    crate::protocol_serde::shape_connection_state::de_connection_state(tokens)?
                                );
                            }
                            "destinationConnectionState" => {
                                builder = builder.set_destination_connection_state(
                                    crate::protocol_serde::shape_connection_state::de_connection_state(tokens)?
                                );
                            }
                            "description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "destinationConfig" => {
                                builder = builder.set_destination_config(
                                    crate::protocol_serde::shape_destination_config::de_destination_config(tokens)?
                                );
                            }
                            "timeoutConfig" => {
                                builder = builder.set_timeout_config(
                                    crate::protocol_serde::shape_timeout_config::de_timeout_config(tokens)?
                                );
                            }
                            "tags" => {
                                builder = builder.set_tags(
                                    crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?
                                );
                            }
                            "createdAt" => {
                                builder = builder.set_created_at(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "lastUpdatedAt" => {
                                builder = builder.set_last_updated_at(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
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

