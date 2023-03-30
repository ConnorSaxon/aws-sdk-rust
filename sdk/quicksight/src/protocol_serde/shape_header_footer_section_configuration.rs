// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_header_footer_section_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HeaderFooterSectionConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.section_id {
        object.key("SectionId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.layout {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Layout").start_object();
        crate::protocol_serde::shape_section_layout_configuration::ser_section_layout_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.style {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Style").start_object();
        crate::protocol_serde::shape_section_style::ser_section_style(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_header_footer_section_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::HeaderFooterSectionConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::header_footer_section_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SectionId" => {
                                builder = builder.set_section_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Layout" => {
                                builder = builder.set_layout(
                                    crate::protocol_serde::shape_section_layout_configuration::de_section_layout_configuration(tokens)?
                                );
                            }
                            "Style" => {
                                builder = builder.set_style(
                                    crate::protocol_serde::shape_section_style::de_section_style(tokens)?
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

