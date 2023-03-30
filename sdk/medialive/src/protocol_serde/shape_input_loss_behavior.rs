// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_input_loss_behavior(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InputLossBehavior) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.black_frame_msec != 0 {
        object.key("blackFrameMsec").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.black_frame_msec).into()));
    }
    if let Some(var_1) = &input.input_loss_image_color {
        object.key("inputLossImageColor").string(var_1.as_str());
    }
    if let Some(var_2) = &input.input_loss_image_slate {
        #[allow(unused_mut)]
        let mut object_3 = object.key("inputLossImageSlate").start_object();
        crate::protocol_serde::shape_input_location::ser_input_location(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.input_loss_image_type {
        object.key("inputLossImageType").string(var_4.as_str());
    }
    if input.repeat_frame_msec != 0 {
        object.key("repeatFrameMsec").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.repeat_frame_msec).into()));
    }
    Ok(())
}

pub(crate) fn de_input_loss_behavior<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::InputLossBehavior>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::input_loss_behavior::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "blackFrameMsec" => {
                                builder = builder.set_black_frame_msec(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "inputLossImageColor" => {
                                builder = builder.set_input_loss_image_color(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "inputLossImageSlate" => {
                                builder = builder.set_input_loss_image_slate(
                                    crate::protocol_serde::shape_input_location::de_input_location(tokens)?
                                );
                            }
                            "inputLossImageType" => {
                                builder = builder.set_input_loss_image_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::InputLossImageType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "repeatFrameMsec" => {
                                builder = builder.set_repeat_frame_msec(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
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

