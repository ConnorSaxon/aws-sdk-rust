// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_code_build_project_environment_registry_credential(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsCodeBuildProjectEnvironmentRegistryCredential) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.credential {
        object.key("Credential").string(var_1.as_str());
    }
    if let Some(var_2) = &input.credential_provider {
        object.key("CredentialProvider").string(var_2.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_code_build_project_environment_registry_credential<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsCodeBuildProjectEnvironmentRegistryCredential>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_code_build_project_environment_registry_credential::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Credential" => {
                                builder = builder.set_credential(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CredentialProvider" => {
                                builder = builder.set_credential_provider(
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

