// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_user_pool_policy_type(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UserPoolPolicyType) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.password_policy {
        #[allow(unused_mut)]
        let mut object_2 = object.key("PasswordPolicy").start_object();
        crate::protocol_serde::shape_password_policy_type::ser_password_policy_type(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_user_pool_policy_type<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::UserPoolPolicyType>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::user_pool_policy_type::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "PasswordPolicy" => {
                                builder = builder.set_password_policy(
                                    crate::protocol_serde::shape_password_policy_type::de_password_policy_type(tokens)?
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

