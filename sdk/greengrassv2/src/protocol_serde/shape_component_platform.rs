// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_component_platform(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ComponentPlatform) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_3 = object.key("attributes").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    Ok(())
}

pub(crate) fn de_component_platform<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ComponentPlatform>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::component_platform::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "attributes" => {
                                builder = builder.set_attributes(
                                    crate::protocol_serde::shape_platform_attributes_map::de_platform_attributes_map(tokens)?
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

