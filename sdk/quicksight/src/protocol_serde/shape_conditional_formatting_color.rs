// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_conditional_formatting_color(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConditionalFormattingColor) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.solid {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Solid").start_object();
        crate::protocol_serde::shape_conditional_formatting_solid_color::ser_conditional_formatting_solid_color(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.gradient {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Gradient").start_object();
        crate::protocol_serde::shape_conditional_formatting_gradient_color::ser_conditional_formatting_gradient_color(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_conditional_formatting_color<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ConditionalFormattingColor>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::conditional_formatting_color::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Solid" => {
                                builder = builder.set_solid(
                                    crate::protocol_serde::shape_conditional_formatting_solid_color::de_conditional_formatting_solid_color(tokens)?
                                );
                            }
                            "Gradient" => {
                                builder = builder.set_gradient(
                                    crate::protocol_serde::shape_conditional_formatting_gradient_color::de_conditional_formatting_gradient_color(tokens)?
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

