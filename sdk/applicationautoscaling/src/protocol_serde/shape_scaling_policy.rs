// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_scaling_policy<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ScalingPolicy>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::scaling_policy::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "PolicyARN" => {
                                builder = builder.set_policy_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "PolicyName" => {
                                builder = builder.set_policy_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ServiceNamespace" => {
                                builder = builder.set_service_namespace(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ServiceNamespace::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ResourceId" => {
                                builder = builder.set_resource_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ScalableDimension" => {
                                builder = builder.set_scalable_dimension(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ScalableDimension::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "PolicyType" => {
                                builder = builder.set_policy_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::PolicyType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "StepScalingPolicyConfiguration" => {
                                builder = builder.set_step_scaling_policy_configuration(
                                    crate::protocol_serde::shape_step_scaling_policy_configuration::de_step_scaling_policy_configuration(tokens)?
                                );
                            }
                            "TargetTrackingScalingPolicyConfiguration" => {
                                builder = builder.set_target_tracking_scaling_policy_configuration(
                                    crate::protocol_serde::shape_target_tracking_scaling_policy_configuration::de_target_tracking_scaling_policy_configuration(tokens)?
                                );
                            }
                            "Alarms" => {
                                builder = builder.set_alarms(
                                    crate::protocol_serde::shape_alarms::de_alarms(tokens)?
                                );
                            }
                            "CreationTime" => {
                                builder = builder.set_creation_time(
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

