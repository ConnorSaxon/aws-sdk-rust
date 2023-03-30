// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_profiler_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ProfilerConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.s3_output_path {
        object.key("S3OutputPath").string(var_1.as_str());
    }
    if let Some(var_2) = &input.profiling_interval_in_milliseconds {
        object.key("ProfilingIntervalInMilliseconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.profiling_parameters {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ProfilingParameters").start_object();
        for (key_5, value_6) in var_3 {
             {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if input.disable_profiler {
        object.key("DisableProfiler").boolean(input.disable_profiler);
    }
    Ok(())
}

pub(crate) fn de_profiler_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ProfilerConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::profiler_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "S3OutputPath" => {
                                builder = builder.set_s3_output_path(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ProfilingIntervalInMilliseconds" => {
                                builder = builder.set_profiling_interval_in_milliseconds(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
                                );
                            }
                            "ProfilingParameters" => {
                                builder = builder.set_profiling_parameters(
                                    crate::protocol_serde::shape_profiling_parameters::de_profiling_parameters(tokens)?
                                );
                            }
                            "DisableProfiler" => {
                                builder = builder.set_disable_profiler(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

