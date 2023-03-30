// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_application_component_detail<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ApplicationComponentDetail>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::application_component_detail::Builder::default();
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
                            "name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "recommendationSet" => {
                                builder = builder.set_recommendation_set(
                                    crate::protocol_serde::shape_recommendation_set::de_recommendation_set(tokens)?
                                );
                            }
                            "analysisStatus" => {
                                builder = builder.set_analysis_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::SrcCodeOrDbAnalysisStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "statusMessage" => {
                                builder = builder.set_status_message(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "listAntipatternSeveritySummary" => {
                                builder = builder.set_list_antipattern_severity_summary(
                                    crate::protocol_serde::shape_list_antipattern_severity_summary::de_list_antipattern_severity_summary(tokens)?
                                );
                            }
                            "databaseConfigDetail" => {
                                builder = builder.set_database_config_detail(
                                    crate::protocol_serde::shape_database_config_detail::de_database_config_detail(tokens)?
                                );
                            }
                            "sourceCodeRepositories" => {
                                builder = builder.set_source_code_repositories(
                                    crate::protocol_serde::shape_source_code_repositories::de_source_code_repositories(tokens)?
                                );
                            }
                            "appType" => {
                                builder = builder.set_app_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AppType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "resourceSubType" => {
                                builder = builder.set_resource_sub_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ResourceSubType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "inclusionStatus" => {
                                builder = builder.set_inclusion_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::InclusionStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "antipatternReportS3Object" => {
                                builder = builder.set_antipattern_report_s3_object(
                                    crate::protocol_serde::shape_s3_object::de_s3_object(tokens)?
                                );
                            }
                            "antipatternReportStatus" => {
                                builder = builder.set_antipattern_report_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AntipatternReportStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "antipatternReportStatusMessage" => {
                                builder = builder.set_antipattern_report_status_message(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "osVersion" => {
                                builder = builder.set_os_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "osDriver" => {
                                builder = builder.set_os_driver(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "lastAnalyzedTimestamp" => {
                                builder = builder.set_last_analyzed_timestamp(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "associatedServerId" => {
                                builder = builder.set_associated_server_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "moreServerAssociationExists" => {
                                builder = builder.set_more_server_association_exists(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "runtimeStatus" => {
                                builder = builder.set_runtime_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::RuntimeAnalysisStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "runtimeStatusMessage" => {
                                builder = builder.set_runtime_status_message(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "appUnitError" => {
                                builder = builder.set_app_unit_error(
                                    crate::protocol_serde::shape_app_unit_error::de_app_unit_error(tokens)?
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

