// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_fleet_capacity_reservation(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::FleetCapacityReservation, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::FleetCapacityReservation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("capacityReservationId") /* CapacityReservationId com.amazonaws.ec2#FleetCapacityReservation$CapacityReservationId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation_id(var_1);
            }
            ,
            s if s.matches("availabilityZoneId") /* AvailabilityZoneId com.amazonaws.ec2#FleetCapacityReservation$AvailabilityZoneId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone_id(var_2);
            }
            ,
            s if s.matches("instanceType") /* InstanceType com.amazonaws.ec2#FleetCapacityReservation$InstanceType */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::model::InstanceType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::InstanceType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_type(var_3);
            }
            ,
            s if s.matches("instancePlatform") /* InstancePlatform com.amazonaws.ec2#FleetCapacityReservation$InstancePlatform */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::model::CapacityReservationInstancePlatform, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::CapacityReservationInstancePlatform::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_platform(var_4);
            }
            ,
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#FleetCapacityReservation$AvailabilityZone */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_5);
            }
            ,
            s if s.matches("totalInstanceCount") /* TotalInstanceCount com.amazonaws.ec2#FleetCapacityReservation$TotalInstanceCount */ =>  {
                let var_6 =
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
                builder = builder.set_total_instance_count(var_6);
            }
            ,
            s if s.matches("fulfilledCapacity") /* FulfilledCapacity com.amazonaws.ec2#FleetCapacityReservation$FulfilledCapacity */ =>  {
                let var_7 =
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
                builder = builder.set_fulfilled_capacity(var_7);
            }
            ,
            s if s.matches("ebsOptimized") /* EbsOptimized com.amazonaws.ec2#FleetCapacityReservation$EbsOptimized */ =>  {
                let var_8 =
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
                builder = builder.set_ebs_optimized(var_8);
            }
            ,
            s if s.matches("createDate") /* CreateDate com.amazonaws.ec2#FleetCapacityReservation$CreateDate */ =>  {
                let var_9 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_create_date(var_9);
            }
            ,
            s if s.matches("weight") /* Weight com.amazonaws.ec2#FleetCapacityReservation$Weight */ =>  {
                let var_10 =
                    Some(
                         {
                            <f64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ec2#DoubleWithConstraints`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_weight(var_10);
            }
            ,
            s if s.matches("priority") /* Priority com.amazonaws.ec2#FleetCapacityReservation$Priority */ =>  {
                let var_11 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#IntegerWithConstraints`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_priority(var_11);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

