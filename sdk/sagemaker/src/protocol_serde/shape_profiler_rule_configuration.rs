// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_profiler_rule_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ProfilerRuleConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.rule_configuration_name {
        object.key("RuleConfigurationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.local_path {
        object.key("LocalPath").string(var_2.as_str());
    }
    if let Some(var_3) = &input.s3_output_path {
        object.key("S3OutputPath").string(var_3.as_str());
    }
    if let Some(var_4) = &input.rule_evaluator_image {
        object.key("RuleEvaluatorImage").string(var_4.as_str());
    }
    if let Some(var_5) = &input.instance_type {
        object.key("InstanceType").string(var_5.as_str());
    }
    if input.volume_size_in_gb != 0 {
        object.key("VolumeSizeInGB").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.volume_size_in_gb).into()));
    }
    if let Some(var_6) = &input.rule_parameters {
        #[allow(unused_mut)]
        let mut object_7 = object.key("RuleParameters").start_object();
        for (key_8, value_9) in var_6 {
             {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    Ok(())
}

pub(crate) fn de_profiler_rule_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ProfilerRuleConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::profiler_rule_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "RuleConfigurationName" => {
                                builder = builder.set_rule_configuration_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "LocalPath" => {
                                builder = builder.set_local_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "S3OutputPath" => {
                                builder = builder.set_s3_output_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RuleEvaluatorImage" => {
                                builder = builder.set_rule_evaluator_image(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "InstanceType" => {
                                builder = builder.set_instance_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ProcessingInstanceType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "VolumeSizeInGB" => {
                                builder = builder.set_volume_size_in_gb(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "RuleParameters" => {
                                builder = builder.set_rule_parameters(
                                    crate::protocol_serde::shape_rule_parameters::de_rule_parameters(tokens)?
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

