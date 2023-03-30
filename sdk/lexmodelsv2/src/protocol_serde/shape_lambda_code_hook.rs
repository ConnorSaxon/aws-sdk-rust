// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lambda_code_hook(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LambdaCodeHook) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.lambda_arn {
        object.key("lambdaARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.code_hook_interface_version {
        object.key("codeHookInterfaceVersion").string(var_2.as_str());
    }
    Ok(())
}

pub(crate) fn de_lambda_code_hook<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::LambdaCodeHook>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::lambda_code_hook::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "lambdaARN" => {
                                builder = builder.set_lambda_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "codeHookInterfaceVersion" => {
                                builder = builder.set_code_hook_interface_version(
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

