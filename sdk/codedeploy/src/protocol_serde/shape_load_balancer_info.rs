// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_load_balancer_info(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LoadBalancerInfo) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.elb_info_list {
        let mut array_2 = object.key("elbInfoList").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_elb_info::ser_elb_info(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.target_group_info_list {
        let mut array_6 = object.key("targetGroupInfoList").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_target_group_info::ser_target_group_info(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.target_group_pair_info_list {
        let mut array_10 = object.key("targetGroupPairInfoList").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_target_group_pair_info::ser_target_group_pair_info(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub(crate) fn de_load_balancer_info<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::LoadBalancerInfo>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::load_balancer_info::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "elbInfoList" => {
                                builder = builder.set_elb_info_list(
                                    crate::protocol_serde::shape_elb_info_list::de_elb_info_list(tokens)?
                                );
                            }
                            "targetGroupInfoList" => {
                                builder = builder.set_target_group_info_list(
                                    crate::protocol_serde::shape_target_group_info_list::de_target_group_info_list(tokens)?
                                );
                            }
                            "targetGroupPairInfoList" => {
                                builder = builder.set_target_group_pair_info_list(
                                    crate::protocol_serde::shape_target_group_pair_info_list::de_target_group_pair_info_list(tokens)?
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

