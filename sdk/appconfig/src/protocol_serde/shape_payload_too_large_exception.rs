// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_payload_too_large_exception_json_err(value: &[u8], mut builder: crate::error::payload_too_large_exception::Builder) -> Result<crate::error::payload_too_large_exception::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Measure" => {
                        builder = builder.set_measure(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::BytesMeasure::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "Limit" => {
                        builder = builder.set_limit(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f32_lossy())
                        );
                    }
                    "Size" => {
                        builder = builder.set_size(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f32_lossy())
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

