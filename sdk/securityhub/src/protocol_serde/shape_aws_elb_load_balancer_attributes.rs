// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_elb_load_balancer_attributes(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsElbLoadBalancerAttributes) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.access_log {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AccessLog").start_object();
        crate::protocol_serde::shape_aws_elb_load_balancer_access_log::ser_aws_elb_load_balancer_access_log(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.connection_draining {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ConnectionDraining").start_object();
        crate::protocol_serde::shape_aws_elb_load_balancer_connection_draining::ser_aws_elb_load_balancer_connection_draining(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.connection_settings {
        #[allow(unused_mut)]
        let mut object_6 = object.key("ConnectionSettings").start_object();
        crate::protocol_serde::shape_aws_elb_load_balancer_connection_settings::ser_aws_elb_load_balancer_connection_settings(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.cross_zone_load_balancing {
        #[allow(unused_mut)]
        let mut object_8 = object.key("CrossZoneLoadBalancing").start_object();
        crate::protocol_serde::shape_aws_elb_load_balancer_cross_zone_load_balancing::ser_aws_elb_load_balancer_cross_zone_load_balancing(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.additional_attributes {
        let mut array_10 = object.key("AdditionalAttributes").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_aws_elb_load_balancer_additional_attribute::ser_aws_elb_load_balancer_additional_attribute(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_elb_load_balancer_attributes<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsElbLoadBalancerAttributes>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_elb_load_balancer_attributes::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AccessLog" => {
                                builder = builder.set_access_log(
                                    crate::protocol_serde::shape_aws_elb_load_balancer_access_log::de_aws_elb_load_balancer_access_log(tokens)?
                                );
                            }
                            "ConnectionDraining" => {
                                builder = builder.set_connection_draining(
                                    crate::protocol_serde::shape_aws_elb_load_balancer_connection_draining::de_aws_elb_load_balancer_connection_draining(tokens)?
                                );
                            }
                            "ConnectionSettings" => {
                                builder = builder.set_connection_settings(
                                    crate::protocol_serde::shape_aws_elb_load_balancer_connection_settings::de_aws_elb_load_balancer_connection_settings(tokens)?
                                );
                            }
                            "CrossZoneLoadBalancing" => {
                                builder = builder.set_cross_zone_load_balancing(
                                    crate::protocol_serde::shape_aws_elb_load_balancer_cross_zone_load_balancing::de_aws_elb_load_balancer_cross_zone_load_balancing(tokens)?
                                );
                            }
                            "AdditionalAttributes" => {
                                builder = builder.set_additional_attributes(
                                    crate::protocol_serde::shape_aws_elb_load_balancer_additional_attribute_list::de_aws_elb_load_balancer_additional_attribute_list(tokens)?
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

