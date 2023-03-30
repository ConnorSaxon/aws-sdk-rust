// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_api_gateway_v2_route_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsApiGatewayV2RouteSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.detailed_metrics_enabled {
        object.key("DetailedMetricsEnabled").boolean(input.detailed_metrics_enabled);
    }
    if let Some(var_1) = &input.logging_level {
        object.key("LoggingLevel").string(var_1.as_str());
    }
    if input.data_trace_enabled {
        object.key("DataTraceEnabled").boolean(input.data_trace_enabled);
    }
    if input.throttling_burst_limit != 0 {
        object.key("ThrottlingBurstLimit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.throttling_burst_limit).into()));
    }
    if input.throttling_rate_limit != 0.0 {
        object.key("ThrottlingRateLimit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.throttling_rate_limit).into()));
    }
    Ok(())
}

pub(crate) fn de_aws_api_gateway_v2_route_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsApiGatewayV2RouteSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_api_gateway_v2_route_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "DetailedMetricsEnabled" => {
                                builder = builder.set_detailed_metrics_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "LoggingLevel" => {
                                builder = builder.set_logging_level(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DataTraceEnabled" => {
                                builder = builder.set_data_trace_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "ThrottlingBurstLimit" => {
                                builder = builder.set_throttling_burst_limit(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "ThrottlingRateLimit" => {
                                builder = builder.set_throttling_rate_limit(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
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

