// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_compliance_execution_summary(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ComplianceExecutionSummary) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.execution_time {
        object.key("ExecutionTime").date_time(var_1, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_2) = &input.execution_id {
        object.key("ExecutionId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.execution_type {
        object.key("ExecutionType").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_compliance_execution_summary<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ComplianceExecutionSummary>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::compliance_execution_summary::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ExecutionTime" => {
                                builder = builder.set_execution_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "ExecutionId" => {
                                builder = builder.set_execution_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ExecutionType" => {
                                builder = builder.set_execution_type(
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

