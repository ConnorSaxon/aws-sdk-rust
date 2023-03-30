// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_redshift_cluster_cluster_parameter_group(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsRedshiftClusterClusterParameterGroup) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster_parameter_status_list {
        let mut array_2 = object.key("ClusterParameterStatusList").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_aws_redshift_cluster_cluster_parameter_status::ser_aws_redshift_cluster_cluster_parameter_status(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.parameter_apply_status {
        object.key("ParameterApplyStatus").string(var_5.as_str());
    }
    if let Some(var_6) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_redshift_cluster_cluster_parameter_group<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsRedshiftClusterClusterParameterGroup>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_redshift_cluster_cluster_parameter_group::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ClusterParameterStatusList" => {
                                builder = builder.set_cluster_parameter_status_list(
                                    crate::protocol_serde::shape_aws_redshift_cluster_cluster_parameter_status_list::de_aws_redshift_cluster_cluster_parameter_status_list(tokens)?
                                );
                            }
                            "ParameterApplyStatus" => {
                                builder = builder.set_parameter_apply_status(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ParameterGroupName" => {
                                builder = builder.set_parameter_group_name(
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

