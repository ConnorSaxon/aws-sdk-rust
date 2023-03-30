// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_time_series_replacements_data_source(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TimeSeriesReplacementsDataSource) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.s3_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("S3Config").start_object();
        crate::protocol_serde::shape_s3_config::ser_s3_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.schema {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Schema").start_object();
        crate::protocol_serde::shape_schema::ser_schema(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.format {
        object.key("Format").string(var_5.as_str());
    }
    if let Some(var_6) = &input.timestamp_format {
        object.key("TimestampFormat").string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_time_series_replacements_data_source<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TimeSeriesReplacementsDataSource>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::time_series_replacements_data_source::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "S3Config" => {
                                builder = builder.set_s3_config(
                                    crate::protocol_serde::shape_s3_config::de_s3_config(tokens)?
                                );
                            }
                            "Schema" => {
                                builder = builder.set_schema(
                                    crate::protocol_serde::shape_schema::de_schema(tokens)?
                                );
                            }
                            "Format" => {
                                builder = builder.set_format(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TimestampFormat" => {
                                builder = builder.set_timestamp_format(
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

