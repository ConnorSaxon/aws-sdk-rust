// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_fraud_detection_result<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::FraudDetectionResult>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::fraud_detection_result::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "FraudDetectionResultId" => {
                                builder = builder.set_fraud_detection_result_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "AudioAggregationStartedAt" => {
                                builder = builder.set_audio_aggregation_started_at(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "AudioAggregationEndedAt" => {
                                builder = builder.set_audio_aggregation_ended_at(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "Configuration" => {
                                builder = builder.set_configuration(
                                    crate::protocol_serde::shape_fraud_detection_configuration::de_fraud_detection_configuration(tokens)?
                                );
                            }
                            "Decision" => {
                                builder = builder.set_decision(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::FraudDetectionDecision::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Reasons" => {
                                builder = builder.set_reasons(
                                    crate::protocol_serde::shape_fraud_detection_reasons::de_fraud_detection_reasons(tokens)?
                                );
                            }
                            "RiskDetails" => {
                                builder = builder.set_risk_details(
                                    crate::protocol_serde::shape_fraud_risk_details::de_fraud_risk_details(tokens)?
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

