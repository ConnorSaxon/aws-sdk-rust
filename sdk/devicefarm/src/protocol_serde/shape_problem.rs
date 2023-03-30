// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_problem<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Problem>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::problem::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "run" => {
                                builder = builder.set_run(
                                    crate::protocol_serde::shape_problem_detail::de_problem_detail(tokens)?
                                );
                            }
                            "job" => {
                                builder = builder.set_job(
                                    crate::protocol_serde::shape_problem_detail::de_problem_detail(tokens)?
                                );
                            }
                            "suite" => {
                                builder = builder.set_suite(
                                    crate::protocol_serde::shape_problem_detail::de_problem_detail(tokens)?
                                );
                            }
                            "test" => {
                                builder = builder.set_test(
                                    crate::protocol_serde::shape_problem_detail::de_problem_detail(tokens)?
                                );
                            }
                            "device" => {
                                builder = builder.set_device(
                                    crate::protocol_serde::shape_device::de_device(tokens)?
                                );
                            }
                            "result" => {
                                builder = builder.set_result(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ExecutionResult::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "message" => {
                                builder = builder.set_message(
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

