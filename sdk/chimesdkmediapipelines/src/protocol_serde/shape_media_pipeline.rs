// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_media_pipeline<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::MediaPipeline>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::media_pipeline::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "MediaCapturePipeline" => {
                                builder = builder.set_media_capture_pipeline(
                                    crate::protocol_serde::shape_media_capture_pipeline::de_media_capture_pipeline(tokens)?
                                );
                            }
                            "MediaLiveConnectorPipeline" => {
                                builder = builder.set_media_live_connector_pipeline(
                                    crate::protocol_serde::shape_media_live_connector_pipeline::de_media_live_connector_pipeline(tokens)?
                                );
                            }
                            "MediaConcatenationPipeline" => {
                                builder = builder.set_media_concatenation_pipeline(
                                    crate::protocol_serde::shape_media_concatenation_pipeline::de_media_concatenation_pipeline(tokens)?
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

