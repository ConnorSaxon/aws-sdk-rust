// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_insight_summary<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::InsightSummary>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::insight_summary::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "InsightId" => {
                                builder = builder.set_insight_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "GroupARN" => {
                                builder = builder.set_group_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "GroupName" => {
                                builder = builder.set_group_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RootCauseServiceId" => {
                                builder = builder.set_root_cause_service_id(
                                    crate::protocol_serde::shape_service_id::de_service_id(tokens)?
                                );
                            }
                            "Categories" => {
                                builder = builder.set_categories(
                                    crate::protocol_serde::shape_insight_category_list::de_insight_category_list(tokens)?
                                );
                            }
                            "State" => {
                                builder = builder.set_state(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::InsightState::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "StartTime" => {
                                builder = builder.set_start_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "EndTime" => {
                                builder = builder.set_end_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "Summary" => {
                                builder = builder.set_summary(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ClientRequestImpactStatistics" => {
                                builder = builder.set_client_request_impact_statistics(
                                    crate::protocol_serde::shape_request_impact_statistics::de_request_impact_statistics(tokens)?
                                );
                            }
                            "RootCauseServiceRequestImpactStatistics" => {
                                builder = builder.set_root_cause_service_request_impact_statistics(
                                    crate::protocol_serde::shape_request_impact_statistics::de_request_impact_statistics(tokens)?
                                );
                            }
                            "TopAnomalousServices" => {
                                builder = builder.set_top_anomalous_services(
                                    crate::protocol_serde::shape_anomalous_service_list::de_anomalous_service_list(tokens)?
                                );
                            }
                            "LastUpdateTime" => {
                                builder = builder.set_last_update_time(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
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

