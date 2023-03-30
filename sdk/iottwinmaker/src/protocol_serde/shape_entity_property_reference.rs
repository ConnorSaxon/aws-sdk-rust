// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_entity_property_reference(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EntityPropertyReference) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.component_name {
        object.key("componentName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.external_id_property {
        #[allow(unused_mut)]
        let mut object_3 = object.key("externalIdProperty").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.entity_id {
        object.key("entityId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.property_name {
        object.key("propertyName").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_entity_property_reference<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::EntityPropertyReference>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::entity_property_reference::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "componentName" => {
                                builder = builder.set_component_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "externalIdProperty" => {
                                builder = builder.set_external_id_property(
                                    crate::protocol_serde::shape_external_id_property::de_external_id_property(tokens)?
                                );
                            }
                            "entityId" => {
                                builder = builder.set_entity_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "propertyName" => {
                                builder = builder.set_property_name(
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

