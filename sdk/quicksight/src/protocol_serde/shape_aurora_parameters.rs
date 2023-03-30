// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aurora_parameters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AuroraParameters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.host {
        object.key("Host").string(var_1.as_str());
    }
     {
        object.key("Port").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.port).into()));
    }
    if let Some(var_2) = &input.database {
        object.key("Database").string(var_2.as_str());
    }
    Ok(())
}

pub(crate) fn de_aurora_parameters<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AuroraParameters>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aurora_parameters::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Host" => {
                                builder = builder.set_host(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Port" => {
                                builder = builder.set_port(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "Database" => {
                                builder = builder.set_database(
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

