// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_open_search_service_domain_node_to_node_encryption_options_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsOpenSearchServiceDomainNodeToNodeEncryptionOptionsDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.enabled {
        object.key("Enabled").boolean(input.enabled);
    }
    Ok(())
}

pub(crate) fn de_aws_open_search_service_domain_node_to_node_encryption_options_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsOpenSearchServiceDomainNodeToNodeEncryptionOptionsDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_open_search_service_domain_node_to_node_encryption_options_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Enabled" => {
                                builder = builder.set_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

