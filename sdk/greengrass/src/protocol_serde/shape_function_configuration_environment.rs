// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_function_configuration_environment(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FunctionConfigurationEnvironment) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.access_sysfs {
        object.key("AccessSysfs").boolean(input.access_sysfs);
    }
    if let Some(var_1) = &input.execution {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Execution").start_object();
        crate::protocol_serde::shape_function_execution_config::ser_function_execution_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.resource_access_policies {
        let mut array_4 = object.key("ResourceAccessPolicies").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_resource_access_policy::ser_resource_access_policy(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.variables {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Variables").start_object();
        for (key_9, value_10) in var_7 {
             {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}

pub(crate) fn de_function_configuration_environment<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::FunctionConfigurationEnvironment>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::function_configuration_environment::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AccessSysfs" => {
                                builder = builder.set_access_sysfs(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "Execution" => {
                                builder = builder.set_execution(
                                    crate::protocol_serde::shape_function_execution_config::de_function_execution_config(tokens)?
                                );
                            }
                            "ResourceAccessPolicies" => {
                                builder = builder.set_resource_access_policies(
                                    crate::protocol_serde::shape___list_of_resource_access_policy::de___list_of_resource_access_policy(tokens)?
                                );
                            }
                            "Variables" => {
                                builder = builder.set_variables(
                                    crate::protocol_serde::shape___map_of__string::de___map_of__string(tokens)?
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

