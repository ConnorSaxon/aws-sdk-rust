// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_command<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Command>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::command::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CommandId" => {
                                builder = builder.set_command_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DocumentName" => {
                                builder = builder.set_document_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DocumentVersion" => {
                                builder = builder.set_document_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Comment" => {
                                builder = builder.set_comment(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ExpiresAfter" => {
                                builder = builder.set_expires_after(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "Parameters" => {
                                builder = builder.set_parameters(
                                    crate::protocol_serde::shape_parameters::de_parameters(tokens)?
                                );
                            }
                            "InstanceIds" => {
                                builder = builder.set_instance_ids(
                                    crate::protocol_serde::shape_instance_id_list::de_instance_id_list(tokens)?
                                );
                            }
                            "Targets" => {
                                builder = builder.set_targets(
                                    crate::protocol_serde::shape_targets::de_targets(tokens)?
                                );
                            }
                            "RequestedDateTime" => {
                                builder = builder.set_requested_date_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "Status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CommandStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "StatusDetails" => {
                                builder = builder.set_status_details(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "OutputS3Region" => {
                                builder = builder.set_output_s3_region(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "OutputS3BucketName" => {
                                builder = builder.set_output_s3_bucket_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "OutputS3KeyPrefix" => {
                                builder = builder.set_output_s3_key_prefix(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "MaxConcurrency" => {
                                builder = builder.set_max_concurrency(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "MaxErrors" => {
                                builder = builder.set_max_errors(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TargetCount" => {
                                builder = builder.set_target_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CompletedCount" => {
                                builder = builder.set_completed_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "ErrorCount" => {
                                builder = builder.set_error_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "DeliveryTimedOutCount" => {
                                builder = builder.set_delivery_timed_out_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "ServiceRole" => {
                                builder = builder.set_service_role(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "NotificationConfig" => {
                                builder = builder.set_notification_config(
                                    crate::protocol_serde::shape_notification_config::de_notification_config(tokens)?
                                );
                            }
                            "CloudWatchOutputConfig" => {
                                builder = builder.set_cloud_watch_output_config(
                                    crate::protocol_serde::shape_cloud_watch_output_config::de_cloud_watch_output_config(tokens)?
                                );
                            }
                            "TimeoutSeconds" => {
                                builder = builder.set_timeout_seconds(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "AlarmConfiguration" => {
                                builder = builder.set_alarm_configuration(
                                    crate::protocol_serde::shape_alarm_configuration::de_alarm_configuration(tokens)?
                                );
                            }
                            "TriggeredAlarms" => {
                                builder = builder.set_triggered_alarms(
                                    crate::protocol_serde::shape_alarm_state_information_list::de_alarm_state_information_list(tokens)?
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

