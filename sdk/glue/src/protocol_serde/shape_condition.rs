// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_condition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Condition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.logical_operator {
        object.key("LogicalOperator").string(var_1.as_str());
    }
    if let Some(var_2) = &input.job_name {
        object.key("JobName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.state {
        object.key("State").string(var_3.as_str());
    }
    if let Some(var_4) = &input.crawler_name {
        object.key("CrawlerName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.crawl_state {
        object.key("CrawlState").string(var_5.as_str());
    }
    Ok(())
}

pub(crate) fn de_condition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Condition>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::condition::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "LogicalOperator" => {
                                builder = builder.set_logical_operator(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::LogicalOperator::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "JobName" => {
                                builder = builder.set_job_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "State" => {
                                builder = builder.set_state(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::JobRunState::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "CrawlerName" => {
                                builder = builder.set_crawler_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CrawlState" => {
                                builder = builder.set_crawl_state(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CrawlState::from(u.as_ref())
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

