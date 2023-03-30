// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_association_batch_request_entry(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CreateAssociationBatchRequestEntry) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.instance_id {
        object.key("InstanceId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Parameters").start_object();
        for (key_5, value_6) in var_3 {
             {
                let mut array_7 = object_4.key(key_5.as_str()).start_array();
                for item_8 in value_6 {
                     {
                        array_7.value().string(item_8.as_str());
                    }
                }
                array_7.finish();
            }
        }
        object_4.finish();
    }
    if let Some(var_9) = &input.automation_target_parameter_name {
        object.key("AutomationTargetParameterName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.document_version {
        object.key("DocumentVersion").string(var_10.as_str());
    }
    if let Some(var_11) = &input.targets {
        let mut array_12 = object.key("Targets").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_target::ser_target(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.schedule_expression {
        object.key("ScheduleExpression").string(var_15.as_str());
    }
    if let Some(var_16) = &input.output_location {
        #[allow(unused_mut)]
        let mut object_17 = object.key("OutputLocation").start_object();
        crate::protocol_serde::shape_instance_association_output_location::ser_instance_association_output_location(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.association_name {
        object.key("AssociationName").string(var_18.as_str());
    }
    if let Some(var_19) = &input.max_errors {
        object.key("MaxErrors").string(var_19.as_str());
    }
    if let Some(var_20) = &input.max_concurrency {
        object.key("MaxConcurrency").string(var_20.as_str());
    }
    if let Some(var_21) = &input.compliance_severity {
        object.key("ComplianceSeverity").string(var_21.as_str());
    }
    if let Some(var_22) = &input.sync_compliance {
        object.key("SyncCompliance").string(var_22.as_str());
    }
    if input.apply_only_at_cron_interval {
        object.key("ApplyOnlyAtCronInterval").boolean(input.apply_only_at_cron_interval);
    }
    if let Some(var_23) = &input.calendar_names {
        let mut array_24 = object.key("CalendarNames").start_array();
        for item_25 in var_23 {
             {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.target_locations {
        let mut array_27 = object.key("TargetLocations").start_array();
        for item_28 in var_26 {
             {
                #[allow(unused_mut)]
                let mut object_29 = array_27.value().start_object();
                crate::protocol_serde::shape_target_location::ser_target_location(&mut object_29, item_28)?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.schedule_offset {
        object.key("ScheduleOffset").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_30).into()));
    }
    if let Some(var_31) = &input.target_maps {
        let mut array_32 = object.key("TargetMaps").start_array();
        for item_33 in var_31 {
             {
                #[allow(unused_mut)]
                let mut object_34 = array_32.value().start_object();
                for (key_35, value_36) in item_33 {
                     {
                        let mut array_37 = object_34.key(key_35.as_str()).start_array();
                        for item_38 in value_36 {
                             {
                                array_37.value().string(item_38.as_str());
                            }
                        }
                        array_37.finish();
                    }
                }
                object_34.finish();
            }
        }
        array_32.finish();
    }
    if let Some(var_39) = &input.alarm_configuration {
        #[allow(unused_mut)]
        let mut object_40 = object.key("AlarmConfiguration").start_object();
        crate::protocol_serde::shape_alarm_configuration::ser_alarm_configuration(&mut object_40, var_39)?;
        object_40.finish();
    }
    Ok(())
}

pub(crate) fn de_create_association_batch_request_entry<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CreateAssociationBatchRequestEntry>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::create_association_batch_request_entry::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "InstanceId" => {
                                builder = builder.set_instance_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Parameters" => {
                                builder = builder.set_parameters(
                                    crate::protocol_serde::shape_parameters::de_parameters(tokens)?
                                );
                            }
                            "AutomationTargetParameterName" => {
                                builder = builder.set_automation_target_parameter_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "DocumentVersion" => {
                                builder = builder.set_document_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Targets" => {
                                builder = builder.set_targets(
                                    crate::protocol_serde::shape_targets::de_targets(tokens)?
                                );
                            }
                            "ScheduleExpression" => {
                                builder = builder.set_schedule_expression(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "OutputLocation" => {
                                builder = builder.set_output_location(
                                    crate::protocol_serde::shape_instance_association_output_location::de_instance_association_output_location(tokens)?
                                );
                            }
                            "AssociationName" => {
                                builder = builder.set_association_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "MaxErrors" => {
                                builder = builder.set_max_errors(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "MaxConcurrency" => {
                                builder = builder.set_max_concurrency(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ComplianceSeverity" => {
                                builder = builder.set_compliance_severity(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AssociationComplianceSeverity::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "SyncCompliance" => {
                                builder = builder.set_sync_compliance(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AssociationSyncCompliance::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ApplyOnlyAtCronInterval" => {
                                builder = builder.set_apply_only_at_cron_interval(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "CalendarNames" => {
                                builder = builder.set_calendar_names(
                                    crate::protocol_serde::shape_calendar_name_or_arn_list::de_calendar_name_or_arn_list(tokens)?
                                );
                            }
                            "TargetLocations" => {
                                builder = builder.set_target_locations(
                                    crate::protocol_serde::shape_target_locations::de_target_locations(tokens)?
                                );
                            }
                            "ScheduleOffset" => {
                                builder = builder.set_schedule_offset(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "TargetMaps" => {
                                builder = builder.set_target_maps(
                                    crate::protocol_serde::shape_target_maps::de_target_maps(tokens)?
                                );
                            }
                            "AlarmConfiguration" => {
                                builder = builder.set_alarm_configuration(
                                    crate::protocol_serde::shape_alarm_configuration::de_alarm_configuration(tokens)?
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

