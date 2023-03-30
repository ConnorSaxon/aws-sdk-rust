// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_sub_slot_setting<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SubSlotSetting>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::sub_slot_setting::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "expression" => {
                                builder = builder.set_expression(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "slotSpecifications" => {
                                builder = builder.set_slot_specifications(
                                    crate::protocol_serde::shape_sub_slot_specification_map::de_sub_slot_specification_map(tokens)?
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

pub fn ser_sub_slot_setting(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SubSlotSetting) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.expression {
        object.key("expression").string(var_1.as_str());
    }
    if let Some(var_2) = &input.slot_specifications {
        #[allow(unused_mut)]
        let mut object_3 = object.key("slotSpecifications").start_object();
        for (key_4, value_5) in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_6 = object_3.key(key_4.as_str()).start_object();
                crate::protocol_serde::shape_specifications::ser_specifications(&mut object_6, value_5)?;
                object_6.finish();
            }
        }
        object_3.finish();
    }
    Ok(())
}

