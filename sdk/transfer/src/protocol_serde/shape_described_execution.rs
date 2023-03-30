// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_described_execution<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DescribedExecution>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::described_execution::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ExecutionId" => {
                                builder = builder.set_execution_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "InitialFileLocation" => {
                                builder = builder.set_initial_file_location(
                                    crate::protocol_serde::shape_file_location::de_file_location(tokens)?
                                );
                            }
                            "ServiceMetadata" => {
                                builder = builder.set_service_metadata(
                                    crate::protocol_serde::shape_service_metadata::de_service_metadata(tokens)?
                                );
                            }
                            "ExecutionRole" => {
                                builder = builder.set_execution_role(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "LoggingConfiguration" => {
                                builder = builder.set_logging_configuration(
                                    crate::protocol_serde::shape_logging_configuration::de_logging_configuration(tokens)?
                                );
                            }
                            "PosixProfile" => {
                                builder = builder.set_posix_profile(
                                    crate::protocol_serde::shape_posix_profile::de_posix_profile(tokens)?
                                );
                            }
                            "Status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ExecutionStatus::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Results" => {
                                builder = builder.set_results(
                                    crate::protocol_serde::shape_execution_results::de_execution_results(tokens)?
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

