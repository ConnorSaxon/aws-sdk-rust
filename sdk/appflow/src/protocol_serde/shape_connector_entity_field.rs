// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_connector_entity_field<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ConnectorEntityField>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::connector_entity_field::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "identifier" => {
                                builder = builder.set_identifier(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "parentIdentifier" => {
                                builder = builder.set_parent_identifier(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "label" => {
                                builder = builder.set_label(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "isPrimaryKey" => {
                                builder = builder.set_is_primary_key(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "defaultValue" => {
                                builder = builder.set_default_value(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "isDeprecated" => {
                                builder = builder.set_is_deprecated(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "supportedFieldTypeDetails" => {
                                builder = builder.set_supported_field_type_details(
                                    crate::protocol_serde::shape_supported_field_type_details::de_supported_field_type_details(tokens)?
                                );
                            }
                            "description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "sourceProperties" => {
                                builder = builder.set_source_properties(
                                    crate::protocol_serde::shape_source_field_properties::de_source_field_properties(tokens)?
                                );
                            }
                            "destinationProperties" => {
                                builder = builder.set_destination_properties(
                                    crate::protocol_serde::shape_destination_field_properties::de_destination_field_properties(tokens)?
                                );
                            }
                            "customProperties" => {
                                builder = builder.set_custom_properties(
                                    crate::protocol_serde::shape_custom_properties::de_custom_properties(tokens)?
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

