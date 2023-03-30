// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_account_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AccountSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::account_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "awsAccountNumber" => {
                                builder = builder.set_aws_account_number(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "unmeteredDevices" => {
                                builder = builder.set_unmetered_devices(
                                    crate::protocol_serde::shape_purchased_devices_map::de_purchased_devices_map(tokens)?
                                );
                            }
                            "unmeteredRemoteAccessDevices" => {
                                builder = builder.set_unmetered_remote_access_devices(
                                    crate::protocol_serde::shape_purchased_devices_map::de_purchased_devices_map(tokens)?
                                );
                            }
                            "maxJobTimeoutMinutes" => {
                                builder = builder.set_max_job_timeout_minutes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "trialMinutes" => {
                                builder = builder.set_trial_minutes(
                                    crate::protocol_serde::shape_trial_minutes::de_trial_minutes(tokens)?
                                );
                            }
                            "maxSlots" => {
                                builder = builder.set_max_slots(
                                    crate::protocol_serde::shape_max_slot_map::de_max_slot_map(tokens)?
                                );
                            }
                            "defaultJobTimeoutMinutes" => {
                                builder = builder.set_default_job_timeout_minutes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "skipAppResign" => {
                                builder = builder.set_skip_app_resign(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

