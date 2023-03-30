// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sql_injection_match_tuple(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SqlInjectionMatchTuple) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.field_to_match {
        #[allow(unused_mut)]
        let mut object_2 = object.key("FieldToMatch").start_object();
        crate::protocol_serde::shape_field_to_match::ser_field_to_match(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.text_transformation {
        object.key("TextTransformation").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_sql_injection_match_tuple<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SqlInjectionMatchTuple>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::sql_injection_match_tuple::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "FieldToMatch" => {
                                builder = builder.set_field_to_match(
                                    crate::protocol_serde::shape_field_to_match::de_field_to_match(tokens)?
                                );
                            }
                            "TextTransformation" => {
                                builder = builder.set_text_transformation(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TextTransformation::from(u.as_ref())
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

