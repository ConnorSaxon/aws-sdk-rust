// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_spot_options(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::SpotOptions, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::SpotOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("allocationStrategy") /* AllocationStrategy com.amazonaws.ec2#SpotOptions$AllocationStrategy */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::model::SpotAllocationStrategy, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::SpotAllocationStrategy::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_allocation_strategy(var_1);
            }
            ,
            s if s.matches("maintenanceStrategies") /* MaintenanceStrategies com.amazonaws.ec2#SpotOptions$MaintenanceStrategies */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_fleet_spot_maintenance_strategies::de_fleet_spot_maintenance_strategies(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_maintenance_strategies(var_2);
            }
            ,
            s if s.matches("instanceInterruptionBehavior") /* InstanceInterruptionBehavior com.amazonaws.ec2#SpotOptions$InstanceInterruptionBehavior */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::model::SpotInstanceInterruptionBehavior, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::SpotInstanceInterruptionBehavior::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_interruption_behavior(var_3);
            }
            ,
            s if s.matches("instancePoolsToUseCount") /* InstancePoolsToUseCount com.amazonaws.ec2#SpotOptions$InstancePoolsToUseCount */ =>  {
                let var_4 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_instance_pools_to_use_count(var_4);
            }
            ,
            s if s.matches("singleInstanceType") /* SingleInstanceType com.amazonaws.ec2#SpotOptions$SingleInstanceType */ =>  {
                let var_5 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_single_instance_type(var_5);
            }
            ,
            s if s.matches("singleAvailabilityZone") /* SingleAvailabilityZone com.amazonaws.ec2#SpotOptions$SingleAvailabilityZone */ =>  {
                let var_6 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_single_availability_zone(var_6);
            }
            ,
            s if s.matches("minTargetCapacity") /* MinTargetCapacity com.amazonaws.ec2#SpotOptions$MinTargetCapacity */ =>  {
                let var_7 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_min_target_capacity(var_7);
            }
            ,
            s if s.matches("maxTotalPrice") /* MaxTotalPrice com.amazonaws.ec2#SpotOptions$MaxTotalPrice */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_max_total_price(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

