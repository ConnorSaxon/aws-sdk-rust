// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vault_access_policy_payload(input: &[u8]) -> Result<crate::model::VaultAccessPolicy, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(input)).peekable();
                        let tokens = &mut tokens_owned;
    let result =
    crate::protocol_serde::shape_vault_access_policy::de_vault_access_policy(tokens)?
    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("expected payload member value"));
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("found more JSON tokens after completing parsing"));
    }
    result
}

pub fn ser_vault_access_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VaultAccessPolicy) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.policy {
        object.key("Policy").string(var_1.as_str());
    }
    Ok(())
}

pub(crate) fn de_vault_access_policy<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::VaultAccessPolicy>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::vault_access_policy::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Policy" => {
                                builder = builder.set_policy(
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

