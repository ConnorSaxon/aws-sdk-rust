// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_launch_template_overrides(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::LaunchTemplateOverrides) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("InstanceType");
    if let Some(var_2) = &input.instance_type {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SpotPrice");
    if let Some(var_4) = &input.spot_price {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("SubnetId");
    if let Some(var_6) = &input.subnet_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("AvailabilityZone");
    if let Some(var_8) = &input.availability_zone {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("WeightedCapacity");
    if let Some(var_10) = &input.weighted_capacity {
        scope_9.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_10).into()));
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Priority");
    if let Some(var_12) = &input.priority {
        scope_11.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_12).into()));
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("InstanceRequirements");
    if let Some(var_14) = &input.instance_requirements {
        crate::protocol_serde::shape_instance_requirements::ser_instance_requirements(scope_13, var_14)?;
    }
    Ok(())
}

pub fn de_launch_template_overrides(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::LaunchTemplateOverrides, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::LaunchTemplateOverrides::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceType") /* InstanceType com.amazonaws.ec2#LaunchTemplateOverrides$InstanceType */ =>  {
                let var_15 =
                    Some(
                        Result::<crate::model::InstanceType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::InstanceType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_type(var_15);
            }
            ,
            s if s.matches("spotPrice") /* SpotPrice com.amazonaws.ec2#LaunchTemplateOverrides$SpotPrice */ =>  {
                let var_16 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_spot_price(var_16);
            }
            ,
            s if s.matches("subnetId") /* SubnetId com.amazonaws.ec2#LaunchTemplateOverrides$SubnetId */ =>  {
                let var_17 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_17);
            }
            ,
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#LaunchTemplateOverrides$AvailabilityZone */ =>  {
                let var_18 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_18);
            }
            ,
            s if s.matches("weightedCapacity") /* WeightedCapacity com.amazonaws.ec2#LaunchTemplateOverrides$WeightedCapacity */ =>  {
                let var_19 =
                    Some(
                         {
                            <f64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ec2#Double`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_weighted_capacity(var_19);
            }
            ,
            s if s.matches("priority") /* Priority com.amazonaws.ec2#LaunchTemplateOverrides$Priority */ =>  {
                let var_20 =
                    Some(
                         {
                            <f64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ec2#Double`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_priority(var_20);
            }
            ,
            s if s.matches("instanceRequirements") /* InstanceRequirements com.amazonaws.ec2#LaunchTemplateOverrides$InstanceRequirements */ =>  {
                let var_21 =
                    Some(
                        crate::protocol_serde::shape_instance_requirements::de_instance_requirements(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_requirements(var_21);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

