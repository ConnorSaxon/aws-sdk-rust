// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_recommendation_job_payload_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RecommendationJobPayloadConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.sample_payload_url {
        object.key("SamplePayloadUrl").string(var_1.as_str());
    }
    if let Some(var_2) = &input.supported_content_types {
        let mut array_3 = object.key("SupportedContentTypes").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub(crate) fn de_recommendation_job_payload_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RecommendationJobPayloadConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::recommendation_job_payload_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SamplePayloadUrl" => {
                                builder = builder.set_sample_payload_url(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SupportedContentTypes" => {
                                builder = builder.set_supported_content_types(
                                    crate::protocol_serde::shape_recommendation_job_supported_content_types::de_recommendation_job_supported_content_types(tokens)?
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

