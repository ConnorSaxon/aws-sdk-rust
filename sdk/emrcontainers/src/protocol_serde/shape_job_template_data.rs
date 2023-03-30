// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_job_template_data(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::JobTemplateData) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.execution_role_arn {
        object.key("executionRoleArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.release_label {
        object.key("releaseLabel").string(var_2.as_str());
    }
    if let Some(var_3) = &input.configuration_overrides {
        #[allow(unused_mut)]
        let mut object_4 = object.key("configurationOverrides").start_object();
        crate::protocol_serde::shape_parametric_configuration_overrides::ser_parametric_configuration_overrides(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.job_driver {
        #[allow(unused_mut)]
        let mut object_6 = object.key("jobDriver").start_object();
        crate::protocol_serde::shape_job_driver::ser_job_driver(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.parameter_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("parameterConfiguration").start_object();
        for (key_9, value_10) in var_7 {
             {
                #[allow(unused_mut)]
                let mut object_11 = object_8.key(key_9.as_str()).start_object();
                crate::protocol_serde::shape_template_parameter_configuration::ser_template_parameter_configuration(&mut object_11, value_10)?;
                object_11.finish();
            }
        }
        object_8.finish();
    }
    if let Some(var_12) = &input.job_tags {
        #[allow(unused_mut)]
        let mut object_13 = object.key("jobTags").start_object();
        for (key_14, value_15) in var_12 {
             {
                object_13.key(key_14.as_str()).string(value_15.as_str());
            }
        }
        object_13.finish();
    }
    Ok(())
}

pub(crate) fn de_job_template_data<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::JobTemplateData>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::job_template_data::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "executionRoleArn" => {
                                builder = builder.set_execution_role_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "releaseLabel" => {
                                builder = builder.set_release_label(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "configurationOverrides" => {
                                builder = builder.set_configuration_overrides(
                                    crate::protocol_serde::shape_parametric_configuration_overrides::de_parametric_configuration_overrides(tokens)?
                                );
                            }
                            "jobDriver" => {
                                builder = builder.set_job_driver(
                                    crate::protocol_serde::shape_job_driver::de_job_driver(tokens)?
                                );
                            }
                            "parameterConfiguration" => {
                                builder = builder.set_parameter_configuration(
                                    crate::protocol_serde::shape_template_parameter_configuration_map::de_template_parameter_configuration_map(tokens)?
                                );
                            }
                            "jobTags" => {
                                builder = builder.set_job_tags(
                                    crate::protocol_serde::shape_tag_map::de_tag_map(tokens)?
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

