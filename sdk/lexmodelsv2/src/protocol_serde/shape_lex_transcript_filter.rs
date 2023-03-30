// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_lex_transcript_filter<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::LexTranscriptFilter>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::lex_transcript_filter::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "dateRangeFilter" => {
                                builder = builder.set_date_range_filter(
                                    crate::protocol_serde::shape_date_range_filter::de_date_range_filter(tokens)?
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

pub fn ser_lex_transcript_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LexTranscriptFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.date_range_filter {
        #[allow(unused_mut)]
        let mut object_2 = object.key("dateRangeFilter").start_object();
        crate::protocol_serde::shape_date_range_filter::ser_date_range_filter(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

