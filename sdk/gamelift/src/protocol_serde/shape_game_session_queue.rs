// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_game_session_queue<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::GameSessionQueue>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::game_session_queue::Builder::default();
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
                            "GameSessionQueueArn" => {
                                builder = builder.set_game_session_queue_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TimeoutInSeconds" => {
                                builder = builder.set_timeout_in_seconds(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "PlayerLatencyPolicies" => {
                                builder = builder.set_player_latency_policies(
                                    crate::protocol_serde::shape_player_latency_policy_list::de_player_latency_policy_list(tokens)?
                                );
                            }
                            "Destinations" => {
                                builder = builder.set_destinations(
                                    crate::protocol_serde::shape_game_session_queue_destination_list::de_game_session_queue_destination_list(tokens)?
                                );
                            }
                            "FilterConfiguration" => {
                                builder = builder.set_filter_configuration(
                                    crate::protocol_serde::shape_filter_configuration::de_filter_configuration(tokens)?
                                );
                            }
                            "PriorityConfiguration" => {
                                builder = builder.set_priority_configuration(
                                    crate::protocol_serde::shape_priority_configuration::de_priority_configuration(tokens)?
                                );
                            }
                            "CustomEventData" => {
                                builder = builder.set_custom_event_data(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "NotificationTarget" => {
                                builder = builder.set_notification_target(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
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

