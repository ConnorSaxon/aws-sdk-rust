// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_authentication_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AuthenticationConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::authentication_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "isBasicAuthSupported" => {
                                builder = builder.set_is_basic_auth_supported(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "isApiKeyAuthSupported" => {
                                builder = builder.set_is_api_key_auth_supported(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "isOAuth2Supported" => {
                                builder = builder.set_is_o_auth2_supported(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "isCustomAuthSupported" => {
                                builder = builder.set_is_custom_auth_supported(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "oAuth2Defaults" => {
                                builder = builder.set_o_auth2_defaults(
                                    crate::protocol_serde::shape_o_auth2_defaults::de_o_auth2_defaults(tokens)?
                                );
                            }
                            "customAuthConfigs" => {
                                builder = builder.set_custom_auth_configs(
                                    crate::protocol_serde::shape_custom_auth_config_list::de_custom_auth_config_list(tokens)?
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

