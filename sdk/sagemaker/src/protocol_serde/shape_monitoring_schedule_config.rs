// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_monitoring_schedule_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MonitoringScheduleConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.schedule_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ScheduleConfig").start_object();
        crate::protocol_serde::shape_schedule_config::ser_schedule_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.monitoring_job_definition {
        #[allow(unused_mut)]
        let mut object_4 = object.key("MonitoringJobDefinition").start_object();
        crate::protocol_serde::shape_monitoring_job_definition::ser_monitoring_job_definition(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.monitoring_job_definition_name {
        object.key("MonitoringJobDefinitionName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.monitoring_type {
        object.key("MonitoringType").string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_monitoring_schedule_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::MonitoringScheduleConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::monitoring_schedule_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ScheduleConfig" => {
                                builder = builder.set_schedule_config(
                                    crate::protocol_serde::shape_schedule_config::de_schedule_config(tokens)?
                                );
                            }
                            "MonitoringJobDefinition" => {
                                builder = builder.set_monitoring_job_definition(
                                    crate::protocol_serde::shape_monitoring_job_definition::de_monitoring_job_definition(tokens)?
                                );
                            }
                            "MonitoringJobDefinitionName" => {
                                builder = builder.set_monitoring_job_definition_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "MonitoringType" => {
                                builder = builder.set_monitoring_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::MonitoringType::from(u.as_ref())
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

