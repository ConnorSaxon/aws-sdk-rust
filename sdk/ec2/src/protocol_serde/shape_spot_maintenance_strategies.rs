// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_spot_maintenance_strategies(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::SpotMaintenanceStrategies) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CapacityRebalance");
    if let Some(var_2) = &input.capacity_rebalance {
        crate::protocol_serde::shape_spot_capacity_rebalance::ser_spot_capacity_rebalance(scope_1, var_2)?;
    }
    Ok(())
}

pub fn de_spot_maintenance_strategies(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::SpotMaintenanceStrategies, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::SpotMaintenanceStrategies::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("capacityRebalance") /* CapacityRebalance com.amazonaws.ec2#SpotMaintenanceStrategies$CapacityRebalance */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_spot_capacity_rebalance::de_spot_capacity_rebalance(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_capacity_rebalance(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

