// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_broker_logs(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BrokerLogs) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cloud_watch_logs {
        #[allow(unused_mut)]
        let mut object_2 = object.key("cloudWatchLogs").start_object();
        crate::protocol_serde::shape_cloud_watch_logs::ser_cloud_watch_logs(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.firehose {
        #[allow(unused_mut)]
        let mut object_4 = object.key("firehose").start_object();
        crate::protocol_serde::shape_firehose::ser_firehose(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.s3 {
        #[allow(unused_mut)]
        let mut object_6 = object.key("s3").start_object();
        crate::protocol_serde::shape_s3::ser_s3(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_broker_logs<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::BrokerLogs>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::broker_logs::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "cloudWatchLogs" => {
                                builder = builder.set_cloud_watch_logs(
                                    crate::protocol_serde::shape_cloud_watch_logs::de_cloud_watch_logs(tokens)?
                                );
                            }
                            "firehose" => {
                                builder = builder.set_firehose(
                                    crate::protocol_serde::shape_firehose::de_firehose(tokens)?
                                );
                            }
                            "s3" => {
                                builder = builder.set_s3(
                                    crate::protocol_serde::shape_s3::de_s3(tokens)?
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

