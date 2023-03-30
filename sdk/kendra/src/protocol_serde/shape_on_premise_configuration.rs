// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_on_premise_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OnPremiseConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.host_url {
        object.key("HostUrl").string(var_1.as_str());
    }
    if let Some(var_2) = &input.organization_name {
        object.key("OrganizationName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ssl_certificate_s3_path {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SslCertificateS3Path").start_object();
        crate::protocol_serde::shape_s3_path::ser_s3_path(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_on_premise_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::OnPremiseConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::on_premise_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "HostUrl" => {
                                builder = builder.set_host_url(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "OrganizationName" => {
                                builder = builder.set_organization_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SslCertificateS3Path" => {
                                builder = builder.set_ssl_certificate_s3_path(
                                    crate::protocol_serde::shape_s3_path::de_s3_path(tokens)?
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

