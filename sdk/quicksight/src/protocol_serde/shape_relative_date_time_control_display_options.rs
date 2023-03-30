// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_relative_date_time_control_display_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RelativeDateTimeControlDisplayOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.title_options {
        #[allow(unused_mut)]
        let mut object_2 = object.key("TitleOptions").start_object();
        crate::protocol_serde::shape_label_options::ser_label_options(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.date_time_format {
        object.key("DateTimeFormat").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_relative_date_time_control_display_options<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RelativeDateTimeControlDisplayOptions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::relative_date_time_control_display_options::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "TitleOptions" => {
                                builder = builder.set_title_options(
                                    crate::protocol_serde::shape_label_options::de_label_options(tokens)?
                                );
                            }
                            "DateTimeFormat" => {
                                builder = builder.set_date_time_format(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
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

