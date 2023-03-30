// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_ecs_service_load_balancers_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsEcsServiceLoadBalancersDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.container_name {
        object.key("ContainerName").string(var_1.as_str());
    }
    if input.container_port != 0 {
        object.key("ContainerPort").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.container_port).into()));
    }
    if let Some(var_2) = &input.load_balancer_name {
        object.key("LoadBalancerName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.target_group_arn {
        object.key("TargetGroupArn").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_ecs_service_load_balancers_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsEcsServiceLoadBalancersDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_ecs_service_load_balancers_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ContainerName" => {
                                builder = builder.set_container_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ContainerPort" => {
                                builder = builder.set_container_port(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "LoadBalancerName" => {
                                builder = builder.set_load_balancer_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TargetGroupArn" => {
                                builder = builder.set_target_group_arn(
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

