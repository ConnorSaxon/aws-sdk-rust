// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_conditional_formatting_custom_icon_condition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConditionalFormattingCustomIconCondition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.expression {
        object.key("Expression").string(var_1.as_str());
    }
    if let Some(var_2) = &input.icon_options {
        #[allow(unused_mut)]
        let mut object_3 = object.key("IconOptions").start_object();
        crate::protocol_serde::shape_conditional_formatting_custom_icon_options::ser_conditional_formatting_custom_icon_options(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.color {
        object.key("Color").string(var_4.as_str());
    }
    if let Some(var_5) = &input.display_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("DisplayConfiguration").start_object();
        crate::protocol_serde::shape_conditional_formatting_icon_display_configuration::ser_conditional_formatting_icon_display_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_conditional_formatting_custom_icon_condition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ConditionalFormattingCustomIconCondition>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::conditional_formatting_custom_icon_condition::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Expression" => {
                                builder = builder.set_expression(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "IconOptions" => {
                                builder = builder.set_icon_options(
                                    crate::protocol_serde::shape_conditional_formatting_custom_icon_options::de_conditional_formatting_custom_icon_options(tokens)?
                                );
                            }
                            "Color" => {
                                builder = builder.set_color(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DisplayConfiguration" => {
                                builder = builder.set_display_configuration(
                                    crate::protocol_serde::shape_conditional_formatting_icon_display_configuration::de_conditional_formatting_icon_display_configuration(tokens)?
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

