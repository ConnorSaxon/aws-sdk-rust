// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_address(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Address) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.city {
        object.key("city").string(var_1.as_str());
    }
    if let Some(var_2) = &input.company {
        object.key("company").string(var_2.as_str());
    }
    if let Some(var_3) = &input.country {
        object.key("country").string(var_3.as_str());
    }
    if let Some(var_4) = &input.name {
        object.key("name").string(var_4.as_str());
    }
    if let Some(var_5) = &input.phone_number {
        object.key("phoneNumber").string(var_5.as_str());
    }
    if let Some(var_6) = &input.postal_code {
        object.key("postalCode").string(var_6.as_str());
    }
    if let Some(var_7) = &input.state_or_province {
        object.key("stateOrProvince").string(var_7.as_str());
    }
    if let Some(var_8) = &input.street1 {
        object.key("street1").string(var_8.as_str());
    }
    if let Some(var_9) = &input.street2 {
        object.key("street2").string(var_9.as_str());
    }
    if let Some(var_10) = &input.street3 {
        object.key("street3").string(var_10.as_str());
    }
    Ok(())
}

pub(crate) fn de_address<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Address>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::address::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "city" => {
                                builder = builder.set_city(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "company" => {
                                builder = builder.set_company(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "country" => {
                                builder = builder.set_country(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "phoneNumber" => {
                                builder = builder.set_phone_number(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "postalCode" => {
                                builder = builder.set_postal_code(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "stateOrProvince" => {
                                builder = builder.set_state_or_province(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "street1" => {
                                builder = builder.set_street1(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "street2" => {
                                builder = builder.set_street2(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "street3" => {
                                builder = builder.set_street3(
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

