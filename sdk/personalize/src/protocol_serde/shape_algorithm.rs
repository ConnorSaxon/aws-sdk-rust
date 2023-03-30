// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_algorithm<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Algorithm>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::algorithm::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "algorithmArn" => {
                                builder = builder.set_algorithm_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "algorithmImage" => {
                                builder = builder.set_algorithm_image(
                                    crate::protocol_serde::shape_algorithm_image::de_algorithm_image(tokens)?
                                );
                            }
                            "defaultHyperParameters" => {
                                builder = builder.set_default_hyper_parameters(
                                    crate::protocol_serde::shape_hyper_parameters::de_hyper_parameters(tokens)?
                                );
                            }
                            "defaultHyperParameterRanges" => {
                                builder = builder.set_default_hyper_parameter_ranges(
                                    crate::protocol_serde::shape_default_hyper_parameter_ranges::de_default_hyper_parameter_ranges(tokens)?
                                );
                            }
                            "defaultResourceConfig" => {
                                builder = builder.set_default_resource_config(
                                    crate::protocol_serde::shape_resource_config::de_resource_config(tokens)?
                                );
                            }
                            "trainingInputMode" => {
                                builder = builder.set_training_input_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "roleArn" => {
                                builder = builder.set_role_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "creationDateTime" => {
                                builder = builder.set_creation_date_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "lastUpdatedDateTime" => {
                                builder = builder.set_last_updated_date_time(
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

