// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_spot_fleet_request_config_data(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::SpotFleetRequestConfigData) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AllocationStrategy");
    if let Some(var_2) = &input.allocation_strategy {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("OnDemandAllocationStrategy");
    if let Some(var_4) = &input.on_demand_allocation_strategy {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("SpotMaintenanceStrategies");
    if let Some(var_6) = &input.spot_maintenance_strategies {
        crate::protocol_serde::shape_spot_maintenance_strategies::ser_spot_maintenance_strategies(scope_5, var_6)?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("ClientToken");
    if let Some(var_8) = &input.client_token {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("ExcessCapacityTerminationPolicy");
    if let Some(var_10) = &input.excess_capacity_termination_policy {
        scope_9.string(var_10.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("FulfilledCapacity");
    if let Some(var_12) = &input.fulfilled_capacity {
        scope_11.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_12).into()));
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("OnDemandFulfilledCapacity");
    if let Some(var_14) = &input.on_demand_fulfilled_capacity {
        scope_13.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_14).into()));
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("IamFleetRole");
    if let Some(var_16) = &input.iam_fleet_role {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("LaunchSpecifications");
    if let Some(var_18) = &input.launch_specifications {
        let mut list_20 = scope_17.start_list(true, Some("item"));
        for item_19 in var_18 {
            #[allow(unused_mut)]
            let mut entry_21 = list_20.entry();
            crate::protocol_serde::shape_spot_fleet_launch_specification::ser_spot_fleet_launch_specification(entry_21, item_19)?;
        }
        list_20.finish();
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("LaunchTemplateConfigs");
    if let Some(var_23) = &input.launch_template_configs {
        let mut list_25 = scope_22.start_list(true, Some("item"));
        for item_24 in var_23 {
            #[allow(unused_mut)]
            let mut entry_26 = list_25.entry();
            crate::protocol_serde::shape_launch_template_config::ser_launch_template_config(entry_26, item_24)?;
        }
        list_25.finish();
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("SpotPrice");
    if let Some(var_28) = &input.spot_price {
        scope_27.string(var_28);
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("TargetCapacity");
    if let Some(var_30) = &input.target_capacity {
        scope_29.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_30).into()));
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("OnDemandTargetCapacity");
    if let Some(var_32) = &input.on_demand_target_capacity {
        scope_31.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_32).into()));
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("OnDemandMaxTotalPrice");
    if let Some(var_34) = &input.on_demand_max_total_price {
        scope_33.string(var_34);
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("SpotMaxTotalPrice");
    if let Some(var_36) = &input.spot_max_total_price {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("TerminateInstancesWithExpiration");
    if let Some(var_38) = &input.terminate_instances_with_expiration {
        scope_37.boolean(*var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("Type");
    if let Some(var_40) = &input.r#type {
        scope_39.string(var_40.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("ValidFrom");
    if let Some(var_42) = &input.valid_from {
        scope_41.date_time(var_42, aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("ValidUntil");
    if let Some(var_44) = &input.valid_until {
        scope_43.date_time(var_44, aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("ReplaceUnhealthyInstances");
    if let Some(var_46) = &input.replace_unhealthy_instances {
        scope_45.boolean(*var_46);
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("InstanceInterruptionBehavior");
    if let Some(var_48) = &input.instance_interruption_behavior {
        scope_47.string(var_48.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("LoadBalancersConfig");
    if let Some(var_50) = &input.load_balancers_config {
        crate::protocol_serde::shape_load_balancers_config::ser_load_balancers_config(scope_49, var_50)?;
    }
    #[allow(unused_mut)]
    let mut scope_51 = writer.prefix("InstancePoolsToUseCount");
    if let Some(var_52) = &input.instance_pools_to_use_count {
        scope_51.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_52).into()));
    }
    #[allow(unused_mut)]
    let mut scope_53 = writer.prefix("Context");
    if let Some(var_54) = &input.context {
        scope_53.string(var_54);
    }
    #[allow(unused_mut)]
    let mut scope_55 = writer.prefix("TargetCapacityUnitType");
    if let Some(var_56) = &input.target_capacity_unit_type {
        scope_55.string(var_56.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_57 = writer.prefix("TagSpecification");
    if let Some(var_58) = &input.tag_specifications {
        let mut list_60 = scope_57.start_list(true, Some("item"));
        for item_59 in var_58 {
            #[allow(unused_mut)]
            let mut entry_61 = list_60.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_61, item_59)?;
        }
        list_60.finish();
    }
    Ok(())
}

pub fn de_spot_fleet_request_config_data(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::SpotFleetRequestConfigData, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::SpotFleetRequestConfigData::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("allocationStrategy") /* AllocationStrategy com.amazonaws.ec2#SpotFleetRequestConfigData$AllocationStrategy */ =>  {
                let var_62 =
                    Some(
                        Result::<crate::model::AllocationStrategy, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::AllocationStrategy::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_allocation_strategy(var_62);
            }
            ,
            s if s.matches("onDemandAllocationStrategy") /* OnDemandAllocationStrategy com.amazonaws.ec2#SpotFleetRequestConfigData$OnDemandAllocationStrategy */ =>  {
                let var_63 =
                    Some(
                        Result::<crate::model::OnDemandAllocationStrategy, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::OnDemandAllocationStrategy::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_on_demand_allocation_strategy(var_63);
            }
            ,
            s if s.matches("spotMaintenanceStrategies") /* SpotMaintenanceStrategies com.amazonaws.ec2#SpotFleetRequestConfigData$SpotMaintenanceStrategies */ =>  {
                let var_64 =
                    Some(
                        crate::protocol_serde::shape_spot_maintenance_strategies::de_spot_maintenance_strategies(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_spot_maintenance_strategies(var_64);
            }
            ,
            s if s.matches("clientToken") /* ClientToken com.amazonaws.ec2#SpotFleetRequestConfigData$ClientToken */ =>  {
                let var_65 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_client_token(var_65);
            }
            ,
            s if s.matches("excessCapacityTerminationPolicy") /* ExcessCapacityTerminationPolicy com.amazonaws.ec2#SpotFleetRequestConfigData$ExcessCapacityTerminationPolicy */ =>  {
                let var_66 =
                    Some(
                        Result::<crate::model::ExcessCapacityTerminationPolicy, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::ExcessCapacityTerminationPolicy::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_excess_capacity_termination_policy(var_66);
            }
            ,
            s if s.matches("fulfilledCapacity") /* FulfilledCapacity com.amazonaws.ec2#SpotFleetRequestConfigData$FulfilledCapacity */ =>  {
                let var_67 =
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
                builder = builder.set_fulfilled_capacity(var_67);
            }
            ,
            s if s.matches("onDemandFulfilledCapacity") /* OnDemandFulfilledCapacity com.amazonaws.ec2#SpotFleetRequestConfigData$OnDemandFulfilledCapacity */ =>  {
                let var_68 =
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
                builder = builder.set_on_demand_fulfilled_capacity(var_68);
            }
            ,
            s if s.matches("iamFleetRole") /* IamFleetRole com.amazonaws.ec2#SpotFleetRequestConfigData$IamFleetRole */ =>  {
                let var_69 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_iam_fleet_role(var_69);
            }
            ,
            s if s.matches("launchSpecifications") /* LaunchSpecifications com.amazonaws.ec2#SpotFleetRequestConfigData$LaunchSpecifications */ =>  {
                let var_70 =
                    Some(
                        crate::protocol_serde::shape_launch_specs_list::de_launch_specs_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_launch_specifications(var_70);
            }
            ,
            s if s.matches("launchTemplateConfigs") /* LaunchTemplateConfigs com.amazonaws.ec2#SpotFleetRequestConfigData$LaunchTemplateConfigs */ =>  {
                let var_71 =
                    Some(
                        crate::protocol_serde::shape_launch_template_config_list::de_launch_template_config_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_launch_template_configs(var_71);
            }
            ,
            s if s.matches("spotPrice") /* SpotPrice com.amazonaws.ec2#SpotFleetRequestConfigData$SpotPrice */ =>  {
                let var_72 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_spot_price(var_72);
            }
            ,
            s if s.matches("targetCapacity") /* TargetCapacity com.amazonaws.ec2#SpotFleetRequestConfigData$TargetCapacity */ =>  {
                let var_73 =
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
                builder = builder.set_target_capacity(var_73);
            }
            ,
            s if s.matches("onDemandTargetCapacity") /* OnDemandTargetCapacity com.amazonaws.ec2#SpotFleetRequestConfigData$OnDemandTargetCapacity */ =>  {
                let var_74 =
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
                builder = builder.set_on_demand_target_capacity(var_74);
            }
            ,
            s if s.matches("onDemandMaxTotalPrice") /* OnDemandMaxTotalPrice com.amazonaws.ec2#SpotFleetRequestConfigData$OnDemandMaxTotalPrice */ =>  {
                let var_75 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_on_demand_max_total_price(var_75);
            }
            ,
            s if s.matches("spotMaxTotalPrice") /* SpotMaxTotalPrice com.amazonaws.ec2#SpotFleetRequestConfigData$SpotMaxTotalPrice */ =>  {
                let var_76 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_spot_max_total_price(var_76);
            }
            ,
            s if s.matches("terminateInstancesWithExpiration") /* TerminateInstancesWithExpiration com.amazonaws.ec2#SpotFleetRequestConfigData$TerminateInstancesWithExpiration */ =>  {
                let var_77 =
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
                builder = builder.set_terminate_instances_with_expiration(var_77);
            }
            ,
            s if s.matches("type") /* Type com.amazonaws.ec2#SpotFleetRequestConfigData$Type */ =>  {
                let var_78 =
                    Some(
                        Result::<crate::model::FleetType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::FleetType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_78);
            }
            ,
            s if s.matches("validFrom") /* ValidFrom com.amazonaws.ec2#SpotFleetRequestConfigData$ValidFrom */ =>  {
                let var_79 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_valid_from(var_79);
            }
            ,
            s if s.matches("validUntil") /* ValidUntil com.amazonaws.ec2#SpotFleetRequestConfigData$ValidUntil */ =>  {
                let var_80 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_valid_until(var_80);
            }
            ,
            s if s.matches("replaceUnhealthyInstances") /* ReplaceUnhealthyInstances com.amazonaws.ec2#SpotFleetRequestConfigData$ReplaceUnhealthyInstances */ =>  {
                let var_81 =
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
                builder = builder.set_replace_unhealthy_instances(var_81);
            }
            ,
            s if s.matches("instanceInterruptionBehavior") /* InstanceInterruptionBehavior com.amazonaws.ec2#SpotFleetRequestConfigData$InstanceInterruptionBehavior */ =>  {
                let var_82 =
                    Some(
                        Result::<crate::model::InstanceInterruptionBehavior, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::InstanceInterruptionBehavior::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_interruption_behavior(var_82);
            }
            ,
            s if s.matches("loadBalancersConfig") /* LoadBalancersConfig com.amazonaws.ec2#SpotFleetRequestConfigData$LoadBalancersConfig */ =>  {
                let var_83 =
                    Some(
                        crate::protocol_serde::shape_load_balancers_config::de_load_balancers_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_load_balancers_config(var_83);
            }
            ,
            s if s.matches("instancePoolsToUseCount") /* InstancePoolsToUseCount com.amazonaws.ec2#SpotFleetRequestConfigData$InstancePoolsToUseCount */ =>  {
                let var_84 =
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
                builder = builder.set_instance_pools_to_use_count(var_84);
            }
            ,
            s if s.matches("context") /* Context com.amazonaws.ec2#SpotFleetRequestConfigData$Context */ =>  {
                let var_85 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_context(var_85);
            }
            ,
            s if s.matches("targetCapacityUnitType") /* TargetCapacityUnitType com.amazonaws.ec2#SpotFleetRequestConfigData$TargetCapacityUnitType */ =>  {
                let var_86 =
                    Some(
                        Result::<crate::model::TargetCapacityUnitType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::TargetCapacityUnitType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_target_capacity_unit_type(var_86);
            }
            ,
            s if s.matches("TagSpecification") /* TagSpecifications com.amazonaws.ec2#SpotFleetRequestConfigData$TagSpecifications */ =>  {
                let var_87 =
                    Some(
                        crate::protocol_serde::shape_tag_specification_list::de_tag_specification_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tag_specifications(var_87);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

