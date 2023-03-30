// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_visibility_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VisibilityConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("SampledRequestsEnabled").boolean(input.sampled_requests_enabled);
    }
     {
        object.key("CloudWatchMetricsEnabled").boolean(input.cloud_watch_metrics_enabled);
    }
    if let Some(var_1) = &input.metric_name {
        object.key("MetricName").string(var_1.as_str());
    }
    Ok(())
}

pub(crate) fn de_visibility_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::VisibilityConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::visibility_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SampledRequestsEnabled" => {
                                builder = builder.set_sampled_requests_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CloudWatchMetricsEnabled" => {
                                builder = builder.set_cloud_watch_metrics_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "MetricName" => {
                                builder = builder.set_metric_name(
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

