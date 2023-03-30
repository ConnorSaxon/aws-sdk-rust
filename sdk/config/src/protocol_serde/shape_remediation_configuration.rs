// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_remediation_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RemediationConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.config_rule_name {
        object.key("ConfigRuleName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_type {
        object.key("TargetType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.target_id {
        object.key("TargetId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.target_version {
        object.key("TargetVersion").string(var_4.as_str());
    }
    if let Some(var_5) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Parameters").start_object();
        for (key_7, value_8) in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_9 = object_6.key(key_7.as_str()).start_object();
                crate::protocol_serde::shape_remediation_parameter_value::ser_remediation_parameter_value(&mut object_9, value_8)?;
                object_9.finish();
            }
        }
        object_6.finish();
    }
    if let Some(var_10) = &input.resource_type {
        object.key("ResourceType").string(var_10.as_str());
    }
    if input.automatic {
        object.key("Automatic").boolean(input.automatic);
    }
    if let Some(var_11) = &input.execution_controls {
        #[allow(unused_mut)]
        let mut object_12 = object.key("ExecutionControls").start_object();
        crate::protocol_serde::shape_execution_controls::ser_execution_controls(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.maximum_automatic_attempts {
        object.key("MaximumAutomaticAttempts").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_13).into()));
    }
    if let Some(var_14) = &input.retry_attempt_seconds {
        object.key("RetryAttemptSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_14).into()));
    }
    if let Some(var_15) = &input.arn {
        object.key("Arn").string(var_15.as_str());
    }
    if let Some(var_16) = &input.created_by_service {
        object.key("CreatedByService").string(var_16.as_str());
    }
    Ok(())
}

pub(crate) fn de_remediation_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RemediationConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::remediation_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ConfigRuleName" => {
                                builder = builder.set_config_rule_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TargetType" => {
                                builder = builder.set_target_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::RemediationTargetType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "TargetId" => {
                                builder = builder.set_target_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TargetVersion" => {
                                builder = builder.set_target_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Parameters" => {
                                builder = builder.set_parameters(
                                    crate::protocol_serde::shape_remediation_parameters::de_remediation_parameters(tokens)?
                                );
                            }
                            "ResourceType" => {
                                builder = builder.set_resource_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Automatic" => {
                                builder = builder.set_automatic(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "ExecutionControls" => {
                                builder = builder.set_execution_controls(
                                    crate::protocol_serde::shape_execution_controls::de_execution_controls(tokens)?
                                );
                            }
                            "MaximumAutomaticAttempts" => {
                                builder = builder.set_maximum_automatic_attempts(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "RetryAttemptSeconds" => {
                                builder = builder.set_retry_attempt_seconds(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
                                );
                            }
                            "Arn" => {
                                builder = builder.set_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CreatedByService" => {
                                builder = builder.set_created_by_service(
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

