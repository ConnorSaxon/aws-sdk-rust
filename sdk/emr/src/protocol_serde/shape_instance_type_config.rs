// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_instance_type_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InstanceTypeConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.instance_type {
        object.key("InstanceType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.weighted_capacity {
        object.key("WeightedCapacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.bid_price {
        object.key("BidPrice").string(var_3.as_str());
    }
    if let Some(var_4) = &input.bid_price_as_percentage_of_on_demand_price {
        object.key("BidPriceAsPercentageOfOnDemandPrice").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_4).into()));
    }
    if let Some(var_5) = &input.ebs_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("EbsConfiguration").start_object();
        crate::protocol_serde::shape_ebs_configuration::ser_ebs_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.configurations {
        let mut array_8 = object.key("Configurations").start_array();
        for item_9 in var_7 {
             {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_configuration::ser_configuration(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.custom_ami_id {
        object.key("CustomAmiId").string(var_11.as_str());
    }
    Ok(())
}

