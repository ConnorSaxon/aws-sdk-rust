// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_filled_map_shape_conditional_formatting(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FilledMapShapeConditionalFormatting) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.field_id {
        object.key("FieldId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.format {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Format").start_object();
        crate::protocol_serde::shape_shape_conditional_format::ser_shape_conditional_format(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

pub(crate) fn de_filled_map_shape_conditional_formatting<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::FilledMapShapeConditionalFormatting>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::filled_map_shape_conditional_formatting::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "FieldId" => {
                                builder = builder.set_field_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Format" => {
                                builder = builder.set_format(
                                    crate::protocol_serde::shape_shape_conditional_format::de_shape_conditional_format(tokens)?
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

