// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_snowflake_connector_profile_properties(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SnowflakeConnectorProfileProperties) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.warehouse {
        object.key("warehouse").string(var_1.as_str());
    }
    if let Some(var_2) = &input.stage {
        object.key("stage").string(var_2.as_str());
    }
    if let Some(var_3) = &input.bucket_name {
        object.key("bucketName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.bucket_prefix {
        object.key("bucketPrefix").string(var_4.as_str());
    }
    if let Some(var_5) = &input.private_link_service_name {
        object.key("privateLinkServiceName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.account_name {
        object.key("accountName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.region {
        object.key("region").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_snowflake_connector_profile_properties<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SnowflakeConnectorProfileProperties>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::snowflake_connector_profile_properties::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "warehouse" => {
                                builder = builder.set_warehouse(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "stage" => {
                                builder = builder.set_stage(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "bucketName" => {
                                builder = builder.set_bucket_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "bucketPrefix" => {
                                builder = builder.set_bucket_prefix(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "privateLinkServiceName" => {
                                builder = builder.set_private_link_service_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "accountName" => {
                                builder = builder.set_account_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "region" => {
                                builder = builder.set_region(
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

