// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_event_data_store_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateEventDataStoreInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_data_store {
        object.key("EventDataStore").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.advanced_event_selectors {
        let mut array_4 = object.key("AdvancedEventSelectors").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_advanced_event_selector::ser_advanced_event_selector(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.multi_region_enabled {
        object.key("MultiRegionEnabled").boolean(*var_7);
    }
    if let Some(var_8) = &input.organization_enabled {
        object.key("OrganizationEnabled").boolean(*var_8);
    }
    if let Some(var_9) = &input.retention_period {
        object.key("RetentionPeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_9).into()));
    }
    if let Some(var_10) = &input.termination_protection_enabled {
        object.key("TerminationProtectionEnabled").boolean(*var_10);
    }
    if let Some(var_11) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_11.as_str());
    }
    Ok(())
}

