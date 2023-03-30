// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_approval_threshold_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ApprovalThresholdPolicy) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.threshold_percentage {
        object.key("ThresholdPercentage").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    if let Some(var_2) = &input.proposal_duration_in_hours {
        object.key("ProposalDurationInHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.threshold_comparator {
        object.key("ThresholdComparator").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_approval_threshold_policy<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ApprovalThresholdPolicy>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::approval_threshold_policy::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ThresholdPercentage" => {
                                builder = builder.set_threshold_percentage(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "ProposalDurationInHours" => {
                                builder = builder.set_proposal_duration_in_hours(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "ThresholdComparator" => {
                                builder = builder.set_threshold_comparator(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ThresholdComparator::from(u.as_ref())
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

