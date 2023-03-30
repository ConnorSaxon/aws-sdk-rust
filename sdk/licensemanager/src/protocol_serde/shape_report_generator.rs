// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_report_generator<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ReportGenerator>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::report_generator::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ReportGeneratorName" => {
                                builder = builder.set_report_generator_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ReportType" => {
                                builder = builder.set_report_type(
                                    crate::protocol_serde::shape_report_type_list::de_report_type_list(tokens)?
                                );
                            }
                            "ReportContext" => {
                                builder = builder.set_report_context(
                                    crate::protocol_serde::shape_report_context::de_report_context(tokens)?
                                );
                            }
                            "ReportFrequency" => {
                                builder = builder.set_report_frequency(
                                    crate::protocol_serde::shape_report_frequency::de_report_frequency(tokens)?
                                );
                            }
                            "LicenseManagerReportGeneratorArn" => {
                                builder = builder.set_license_manager_report_generator_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "LastRunStatus" => {
                                builder = builder.set_last_run_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "LastRunFailureReason" => {
                                builder = builder.set_last_run_failure_reason(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "LastReportGenerationTime" => {
                                builder = builder.set_last_report_generation_time(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ReportCreatorAccount" => {
                                builder = builder.set_report_creator_account(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "S3Location" => {
                                builder = builder.set_s3_location(
                                    crate::protocol_serde::shape_s3_location::de_s3_location(tokens)?
                                );
                            }
                            "CreateTime" => {
                                builder = builder.set_create_time(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Tags" => {
                                builder = builder.set_tags(
                                    crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?
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

