// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_proxy_session<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ProxySession>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::proxy_session::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "VoiceConnectorId" => {
                                builder = builder.set_voice_connector_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ProxySessionId" => {
                                builder = builder.set_proxy_session_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ProxySessionStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ExpiryMinutes" => {
                                builder = builder.set_expiry_minutes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "Capabilities" => {
                                builder = builder.set_capabilities(
                                    crate::protocol_serde::shape_capability_list::de_capability_list(tokens)?
                                );
                            }
                            "CreatedTimestamp" => {
                                builder = builder.set_created_timestamp(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::DateTime)?
                                );
                            }
                            "UpdatedTimestamp" => {
                                builder = builder.set_updated_timestamp(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::DateTime)?
                                );
                            }
                            "EndedTimestamp" => {
                                builder = builder.set_ended_timestamp(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::DateTime)?
                                );
                            }
                            "Participants" => {
                                builder = builder.set_participants(
                                    crate::protocol_serde::shape_participants::de_participants(tokens)?
                                );
                            }
                            "NumberSelectionBehavior" => {
                                builder = builder.set_number_selection_behavior(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::NumberSelectionBehavior::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "GeoMatchLevel" => {
                                builder = builder.set_geo_match_level(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::GeoMatchLevel::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "GeoMatchParams" => {
                                builder = builder.set_geo_match_params(
                                    crate::protocol_serde::shape_geo_match_params::de_geo_match_params(tokens)?
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

