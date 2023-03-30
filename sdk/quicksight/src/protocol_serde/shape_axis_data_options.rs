// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_axis_data_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AxisDataOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.numeric_axis_options {
        #[allow(unused_mut)]
        let mut object_2 = object.key("NumericAxisOptions").start_object();
        crate::protocol_serde::shape_numeric_axis_options::ser_numeric_axis_options(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.date_axis_options {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DateAxisOptions").start_object();
        crate::protocol_serde::shape_date_axis_options::ser_date_axis_options(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_axis_data_options<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AxisDataOptions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::axis_data_options::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "NumericAxisOptions" => {
                                builder = builder.set_numeric_axis_options(
                                    crate::protocol_serde::shape_numeric_axis_options::de_numeric_axis_options(tokens)?
                                );
                            }
                            "DateAxisOptions" => {
                                builder = builder.set_date_axis_options(
                                    crate::protocol_serde::shape_date_axis_options::de_date_axis_options(tokens)?
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

