// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_step_functions_action<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::StepFunctionsAction>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::step_functions_action::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "executionNamePrefix" => {
                                builder = builder.set_execution_name_prefix(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "stateMachineName" => {
                                builder = builder.set_state_machine_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "roleArn" => {
                                builder = builder.set_role_arn(
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

pub fn ser_step_functions_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StepFunctionsAction) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.execution_name_prefix {
        object.key("executionNamePrefix").string(var_1.as_str());
    }
    if let Some(var_2) = &input.state_machine_name {
        object.key("stateMachineName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.role_arn {
        object.key("roleArn").string(var_3.as_str());
    }
    Ok(())
}

