// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_usage_statistics<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::UsageStatistics>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::usage_statistics::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "sumByAccount" => {
                                builder = builder.set_sum_by_account(
                                    crate::protocol_serde::shape_usage_account_result_list::de_usage_account_result_list(tokens)?
                                );
                            }
                            "sumByDataSource" => {
                                builder = builder.set_sum_by_data_source(
                                    crate::protocol_serde::shape_usage_data_source_result_list::de_usage_data_source_result_list(tokens)?
                                );
                            }
                            "sumByResource" => {
                                builder = builder.set_sum_by_resource(
                                    crate::protocol_serde::shape_usage_resource_result_list::de_usage_resource_result_list(tokens)?
                                );
                            }
                            "topResources" => {
                                builder = builder.set_top_resources(
                                    crate::protocol_serde::shape_usage_resource_result_list::de_usage_resource_result_list(tokens)?
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

