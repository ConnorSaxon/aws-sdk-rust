// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_build_batch<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::BuildBatch>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::build_batch::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "id" => {
                                builder = builder.set_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "arn" => {
                                builder = builder.set_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "startTime" => {
                                builder = builder.set_start_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "endTime" => {
                                builder = builder.set_end_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "currentPhase" => {
                                builder = builder.set_current_phase(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "buildBatchStatus" => {
                                builder = builder.set_build_batch_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::StatusType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "sourceVersion" => {
                                builder = builder.set_source_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "resolvedSourceVersion" => {
                                builder = builder.set_resolved_source_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "projectName" => {
                                builder = builder.set_project_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "phases" => {
                                builder = builder.set_phases(
                                    crate::protocol_serde::shape_build_batch_phases::de_build_batch_phases(tokens)?
                                );
                            }
                            "source" => {
                                builder = builder.set_source(
                                    crate::protocol_serde::shape_project_source::de_project_source(tokens)?
                                );
                            }
                            "secondarySources" => {
                                builder = builder.set_secondary_sources(
                                    crate::protocol_serde::shape_project_sources::de_project_sources(tokens)?
                                );
                            }
                            "secondarySourceVersions" => {
                                builder = builder.set_secondary_source_versions(
                                    crate::protocol_serde::shape_project_secondary_source_versions::de_project_secondary_source_versions(tokens)?
                                );
                            }
                            "artifacts" => {
                                builder = builder.set_artifacts(
                                    crate::protocol_serde::shape_build_artifacts::de_build_artifacts(tokens)?
                                );
                            }
                            "secondaryArtifacts" => {
                                builder = builder.set_secondary_artifacts(
                                    crate::protocol_serde::shape_build_artifacts_list::de_build_artifacts_list(tokens)?
                                );
                            }
                            "cache" => {
                                builder = builder.set_cache(
                                    crate::protocol_serde::shape_project_cache::de_project_cache(tokens)?
                                );
                            }
                            "environment" => {
                                builder = builder.set_environment(
                                    crate::protocol_serde::shape_project_environment::de_project_environment(tokens)?
                                );
                            }
                            "serviceRole" => {
                                builder = builder.set_service_role(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "logConfig" => {
                                builder = builder.set_log_config(
                                    crate::protocol_serde::shape_logs_config::de_logs_config(tokens)?
                                );
                            }
                            "buildTimeoutInMinutes" => {
                                builder = builder.set_build_timeout_in_minutes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "queuedTimeoutInMinutes" => {
                                builder = builder.set_queued_timeout_in_minutes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "complete" => {
                                builder = builder.set_complete(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "initiator" => {
                                builder = builder.set_initiator(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "vpcConfig" => {
                                builder = builder.set_vpc_config(
                                    crate::protocol_serde::shape_vpc_config::de_vpc_config(tokens)?
                                );
                            }
                            "encryptionKey" => {
                                builder = builder.set_encryption_key(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "buildBatchNumber" => {
                                builder = builder.set_build_batch_number(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
                                );
                            }
                            "fileSystemLocations" => {
                                builder = builder.set_file_system_locations(
                                    crate::protocol_serde::shape_project_file_system_locations::de_project_file_system_locations(tokens)?
                                );
                            }
                            "buildBatchConfig" => {
                                builder = builder.set_build_batch_config(
                                    crate::protocol_serde::shape_project_build_batch_config::de_project_build_batch_config(tokens)?
                                );
                            }
                            "buildGroups" => {
                                builder = builder.set_build_groups(
                                    crate::protocol_serde::shape_build_groups::de_build_groups(tokens)?
                                );
                            }
                            "debugSessionEnabled" => {
                                builder = builder.set_debug_session_enabled(
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

