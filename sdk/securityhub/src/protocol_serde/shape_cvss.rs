// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cvss(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Cvss) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.version {
        object.key("Version").string(var_1.as_str());
    }
    if input.base_score != 0.0 {
        object.key("BaseScore").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.base_score).into()));
    }
    if let Some(var_2) = &input.base_vector {
        object.key("BaseVector").string(var_2.as_str());
    }
    if let Some(var_3) = &input.source {
        object.key("Source").string(var_3.as_str());
    }
    if let Some(var_4) = &input.adjustments {
        let mut array_5 = object.key("Adjustments").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_adjustment::ser_adjustment(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub(crate) fn de_cvss<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Cvss>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::cvss::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Version" => {
                                builder = builder.set_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "BaseScore" => {
                                builder = builder.set_base_score(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                );
                            }
                            "BaseVector" => {
                                builder = builder.set_base_vector(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Source" => {
                                builder = builder.set_source(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Adjustments" => {
                                builder = builder.set_adjustments(
                                    crate::protocol_serde::shape_adjustment_list::de_adjustment_list(tokens)?
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

