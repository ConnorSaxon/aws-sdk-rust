// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_automated_abr_rule(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AutomatedAbrRule) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.allowed_renditions {
        let mut array_2 = object.key("allowedRenditions").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_allowed_rendition_size::ser_allowed_rendition_size(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.force_include_renditions {
        let mut array_6 = object.key("forceIncludeRenditions").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_force_include_rendition_size::ser_force_include_rendition_size(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.min_bottom_rendition_size {
        #[allow(unused_mut)]
        let mut object_10 = object.key("minBottomRenditionSize").start_object();
        crate::protocol_serde::shape_min_bottom_rendition_size::ser_min_bottom_rendition_size(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.min_top_rendition_size {
        #[allow(unused_mut)]
        let mut object_12 = object.key("minTopRenditionSize").start_object();
        crate::protocol_serde::shape_min_top_rendition_size::ser_min_top_rendition_size(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.r#type {
        object.key("type").string(var_13.as_str());
    }
    Ok(())
}

pub(crate) fn de_automated_abr_rule<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AutomatedAbrRule>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::automated_abr_rule::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "allowedRenditions" => {
                                builder = builder.set_allowed_renditions(
                                    crate::protocol_serde::shape___list_of_allowed_rendition_size::de___list_of_allowed_rendition_size(tokens)?
                                );
                            }
                            "forceIncludeRenditions" => {
                                builder = builder.set_force_include_renditions(
                                    crate::protocol_serde::shape___list_of_force_include_rendition_size::de___list_of_force_include_rendition_size(tokens)?
                                );
                            }
                            "minBottomRenditionSize" => {
                                builder = builder.set_min_bottom_rendition_size(
                                    crate::protocol_serde::shape_min_bottom_rendition_size::de_min_bottom_rendition_size(tokens)?
                                );
                            }
                            "minTopRenditionSize" => {
                                builder = builder.set_min_top_rendition_size(
                                    crate::protocol_serde::shape_min_top_rendition_size::de_min_top_rendition_size(tokens)?
                                );
                            }
                            "type" => {
                                builder = builder.set_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::RuleType::from(u.as_ref())
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

