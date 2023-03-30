// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_organization_event_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OrganizationEventFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_type_codes {
        let mut array_2 = object.key("eventTypeCodes").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.aws_account_ids {
        let mut array_5 = object.key("awsAccountIds").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.services {
        let mut array_8 = object.key("services").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.regions {
        let mut array_11 = object.key("regions").start_array();
        for item_12 in var_10 {
             {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.start_time {
        #[allow(unused_mut)]
        let mut object_14 = object.key("startTime").start_object();
        crate::protocol_serde::shape_date_time_range::ser_date_time_range(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.end_time {
        #[allow(unused_mut)]
        let mut object_16 = object.key("endTime").start_object();
        crate::protocol_serde::shape_date_time_range::ser_date_time_range(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.last_updated_time {
        #[allow(unused_mut)]
        let mut object_18 = object.key("lastUpdatedTime").start_object();
        crate::protocol_serde::shape_date_time_range::ser_date_time_range(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.entity_arns {
        let mut array_20 = object.key("entityArns").start_array();
        for item_21 in var_19 {
             {
                array_20.value().string(item_21.as_str());
            }
        }
        array_20.finish();
    }
    if let Some(var_22) = &input.entity_values {
        let mut array_23 = object.key("entityValues").start_array();
        for item_24 in var_22 {
             {
                array_23.value().string(item_24.as_str());
            }
        }
        array_23.finish();
    }
    if let Some(var_25) = &input.event_type_categories {
        let mut array_26 = object.key("eventTypeCategories").start_array();
        for item_27 in var_25 {
             {
                array_26.value().string(item_27.as_str());
            }
        }
        array_26.finish();
    }
    if let Some(var_28) = &input.event_status_codes {
        let mut array_29 = object.key("eventStatusCodes").start_array();
        for item_30 in var_28 {
             {
                array_29.value().string(item_30.as_str());
            }
        }
        array_29.finish();
    }
    Ok(())
}

