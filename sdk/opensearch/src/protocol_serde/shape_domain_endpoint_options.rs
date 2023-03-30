// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_domain_endpoint_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DomainEndpointOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.enforce_https {
        object.key("EnforceHTTPS").boolean(*var_1);
    }
    if let Some(var_2) = &input.tls_security_policy {
        object.key("TLSSecurityPolicy").string(var_2.as_str());
    }
    if let Some(var_3) = &input.custom_endpoint_enabled {
        object.key("CustomEndpointEnabled").boolean(*var_3);
    }
    if let Some(var_4) = &input.custom_endpoint {
        object.key("CustomEndpoint").string(var_4.as_str());
    }
    if let Some(var_5) = &input.custom_endpoint_certificate_arn {
        object.key("CustomEndpointCertificateArn").string(var_5.as_str());
    }
    Ok(())
}

pub(crate) fn de_domain_endpoint_options<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DomainEndpointOptions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::domain_endpoint_options::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "EnforceHTTPS" => {
                                builder = builder.set_enforce_https(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "TLSSecurityPolicy" => {
                                builder = builder.set_tls_security_policy(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TlsSecurityPolicy::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "CustomEndpointEnabled" => {
                                builder = builder.set_custom_endpoint_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CustomEndpoint" => {
                                builder = builder.set_custom_endpoint(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CustomEndpointCertificateArn" => {
                                builder = builder.set_custom_endpoint_certificate_arn(
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

