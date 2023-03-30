// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_table_field_link_content_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TableFieldLinkContentConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.custom_text_content {
        #[allow(unused_mut)]
        let mut object_2 = object.key("CustomTextContent").start_object();
        crate::protocol_serde::shape_table_field_custom_text_content::ser_table_field_custom_text_content(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.custom_icon_content {
        #[allow(unused_mut)]
        let mut object_4 = object.key("CustomIconContent").start_object();
        crate::protocol_serde::shape_table_field_custom_icon_content::ser_table_field_custom_icon_content(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_table_field_link_content_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TableFieldLinkContentConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::table_field_link_content_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CustomTextContent" => {
                                builder = builder.set_custom_text_content(
                                    crate::protocol_serde::shape_table_field_custom_text_content::de_table_field_custom_text_content(tokens)?
                                );
                            }
                            "CustomIconContent" => {
                                builder = builder.set_custom_icon_content(
                                    crate::protocol_serde::shape_table_field_custom_icon_content::de_table_field_custom_icon_content(tokens)?
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

