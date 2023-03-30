// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ip_set_reference_statement(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::IpSetReferenceStatement) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("ARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ip_set_forwarded_ip_config {
        #[allow(unused_mut)]
        let mut object_3 = object.key("IPSetForwardedIPConfig").start_object();
        crate::protocol_serde::shape_ip_set_forwarded_ip_config::ser_ip_set_forwarded_ip_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

pub(crate) fn de_ip_set_reference_statement<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::IpSetReferenceStatement>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::ip_set_reference_statement::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ARN" => {
                                builder = builder.set_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "IPSetForwardedIPConfig" => {
                                builder = builder.set_ip_set_forwarded_ip_config(
                                    crate::protocol_serde::shape_ip_set_forwarded_ip_config::de_ip_set_forwarded_ip_config(tokens)?
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

