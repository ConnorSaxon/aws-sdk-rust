// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rule(object_5: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Rule) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::Rule::NonTalkTimeFilter(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_1 = object_5.key("NonTalkTimeFilter").start_object();
                crate::protocol_serde::shape_non_talk_time_filter::ser_non_talk_time_filter(&mut object_1, inner)?;
                object_1.finish();
            }
        },
        crate::model::Rule::InterruptionFilter(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_2 = object_5.key("InterruptionFilter").start_object();
                crate::protocol_serde::shape_interruption_filter::ser_interruption_filter(&mut object_2, inner)?;
                object_2.finish();
            }
        },
        crate::model::Rule::TranscriptFilter(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_3 = object_5.key("TranscriptFilter").start_object();
                crate::protocol_serde::shape_transcript_filter::ser_transcript_filter(&mut object_3, inner)?;
                object_3.finish();
            }
        },
        crate::model::Rule::SentimentFilter(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_4 = object_5.key("SentimentFilter").start_object();
                crate::protocol_serde::shape_sentiment_filter::ser_sentiment_filter(&mut object_4, inner)?;
                object_4.finish();
            }
        },
        crate::model::Rule::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("Rule"))
    }
    Ok(())
}

pub(crate) fn de_rule<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Rule>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
                                Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        if variant.is_some() {
                                                            return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("encountered mixed variants in union"));
                                                        }
                        variant = match key.to_unescaped()?.as_ref() {
                            "NonTalkTimeFilter" => {
                                Some(crate::model::Rule::NonTalkTimeFilter(
                                    crate::protocol_serde::shape_non_talk_time_filter::de_non_talk_time_filter(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'NonTalkTimeFilter' cannot be null"))?
                                ))
                            }
                            "InterruptionFilter" => {
                                Some(crate::model::Rule::InterruptionFilter(
                                    crate::protocol_serde::shape_interruption_filter::de_interruption_filter(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'InterruptionFilter' cannot be null"))?
                                ))
                            }
                            "TranscriptFilter" => {
                                Some(crate::model::Rule::TranscriptFilter(
                                    crate::protocol_serde::shape_transcript_filter::de_transcript_filter(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'TranscriptFilter' cannot be null"))?
                                ))
                            }
                            "SentimentFilter" => {
                                Some(crate::model::Rule::SentimentFilter(
                                    crate::protocol_serde::shape_sentiment_filter::de_sentiment_filter(tokens)?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'SentimentFilter' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                      aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::model::Rule::Unknown)
                                                                    }
                        };
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
        }
        _ => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
    }
    Ok(variant)
}

