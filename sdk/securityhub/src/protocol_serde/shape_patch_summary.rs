// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_patch_summary(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PatchSummary) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("Id").string(var_1.as_str());
    }
    if input.installed_count != 0 {
        object.key("InstalledCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.installed_count).into()));
    }
    if input.missing_count != 0 {
        object.key("MissingCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.missing_count).into()));
    }
    if input.failed_count != 0 {
        object.key("FailedCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.failed_count).into()));
    }
    if input.installed_other_count != 0 {
        object.key("InstalledOtherCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.installed_other_count).into()));
    }
    if input.installed_rejected_count != 0 {
        object.key("InstalledRejectedCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.installed_rejected_count).into()));
    }
    if input.installed_pending_reboot != 0 {
        object.key("InstalledPendingReboot").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.installed_pending_reboot).into()));
    }
    if let Some(var_2) = &input.operation_start_time {
        object.key("OperationStartTime").string(var_2.as_str());
    }
    if let Some(var_3) = &input.operation_end_time {
        object.key("OperationEndTime").string(var_3.as_str());
    }
    if let Some(var_4) = &input.reboot_option {
        object.key("RebootOption").string(var_4.as_str());
    }
    if let Some(var_5) = &input.operation {
        object.key("Operation").string(var_5.as_str());
    }
    Ok(())
}

pub(crate) fn de_patch_summary<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PatchSummary>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::patch_summary::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Id" => {
                                builder = builder.set_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "InstalledCount" => {
                                builder = builder.set_installed_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "MissingCount" => {
                                builder = builder.set_missing_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "FailedCount" => {
                                builder = builder.set_failed_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "InstalledOtherCount" => {
                                builder = builder.set_installed_other_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "InstalledRejectedCount" => {
                                builder = builder.set_installed_rejected_count(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "InstalledPendingReboot" => {
                                builder = builder.set_installed_pending_reboot(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "OperationStartTime" => {
                                builder = builder.set_operation_start_time(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "OperationEndTime" => {
                                builder = builder.set_operation_end_time(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RebootOption" => {
                                builder = builder.set_reboot_option(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Operation" => {
                                builder = builder.set_operation(
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

