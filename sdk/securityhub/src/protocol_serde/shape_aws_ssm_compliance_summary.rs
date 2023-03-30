// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_ssm_compliance_summary(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsSsmComplianceSummary) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.status {
        object.key("Status").string(var_1.as_str());
    }
    if input.compliant_critical_count != 0 {
        object.key("CompliantCriticalCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.compliant_critical_count).into()));
    }
    if input.compliant_high_count != 0 {
        object.key("CompliantHighCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.compliant_high_count).into()));
    }
    if input.compliant_medium_count != 0 {
        object.key("CompliantMediumCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.compliant_medium_count).into()));
    }
    if let Some(var_2) = &input.execution_type {
        object.key("ExecutionType").string(var_2.as_str());
    }
    if input.non_compliant_critical_count != 0 {
        object.key("NonCompliantCriticalCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.non_compliant_critical_count).into()));
    }
    if input.compliant_informational_count != 0 {
        object.key("CompliantInformationalCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.compliant_informational_count).into()));
    }
    if input.non_compliant_informational_count != 0 {
        object.key("NonCompliantInformationalCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.non_compliant_informational_count).into()));
    }
    if input.compliant_unspecified_count != 0 {
        object.key("CompliantUnspecifiedCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.compliant_unspecified_count).into()));
    }
    if input.non_compliant_low_count != 0 {
        object.key("NonCompliantLowCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.non_compliant_low_count).into()));
    }
    if input.non_compliant_high_count != 0 {
        object.key("NonCompliantHighCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.non_compliant_high_count).into()));
    }
    if input.compliant_low_count != 0 {
        object.key("CompliantLowCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.compliant_low_count).into()));
    }
    if let Some(var_3) = &input.compliance_type {
        object.key("ComplianceType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.patch_baseline_id {
        object.key("PatchBaselineId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.overall_severity {
        object.key("OverallSeverity").string(var_5.as_str());
    }
    if input.non_compliant_medium_count != 0 {
        object.key("NonCompliantMediumCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.non_compliant_medium_count).into()));
    }
    if input.non_compliant_unspecified_count != 0 {
        object.key("NonCompliantUnspecifiedCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.non_compliant_unspecified_count).into()));
    }
    if let Some(var_6) = &input.patch_group {
        object.key("PatchGroup").string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_ssm_compliance_summary<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsSsmComplianceSummary>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_ssm_compliance_summary::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Status" => {
                                builder = builder.set_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "CompliantCriticalCount" => {
                                builder = builder.set_compliant_critical_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CompliantHighCount" => {
                                builder = builder.set_compliant_high_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CompliantMediumCount" => {
                                builder = builder.set_compliant_medium_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
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
                            "NonCompliantCriticalCount" => {
                                builder = builder.set_non_compliant_critical_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CompliantInformationalCount" => {
                                builder = builder.set_compliant_informational_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "NonCompliantInformationalCount" => {
                                builder = builder.set_non_compliant_informational_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CompliantUnspecifiedCount" => {
                                builder = builder.set_compliant_unspecified_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "NonCompliantLowCount" => {
                                builder = builder.set_non_compliant_low_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "NonCompliantHighCount" => {
                                builder = builder.set_non_compliant_high_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "CompliantLowCount" => {
                                builder = builder.set_compliant_low_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "ComplianceType" => {
                                builder = builder.set_compliance_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "PatchBaselineId" => {
                                builder = builder.set_patch_baseline_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "OverallSeverity" => {
                                builder = builder.set_overall_severity(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "NonCompliantMediumCount" => {
                                builder = builder.set_non_compliant_medium_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "NonCompliantUnspecifiedCount" => {
                                builder = builder.set_non_compliant_unspecified_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "PatchGroup" => {
                                builder = builder.set_patch_group(
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

