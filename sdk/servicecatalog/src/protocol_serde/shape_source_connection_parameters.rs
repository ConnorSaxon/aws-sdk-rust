// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_source_connection_parameters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SourceConnectionParameters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.code_star {
        #[allow(unused_mut)]
        let mut object_2 = object.key("CodeStar").start_object();
        crate::protocol_serde::shape_code_star_parameters::ser_code_star_parameters(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_source_connection_parameters<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SourceConnectionParameters>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::source_connection_parameters::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CodeStar" => {
                                builder = builder.set_code_star(
                                    crate::protocol_serde::shape_code_star_parameters::de_code_star_parameters(tokens)?
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

