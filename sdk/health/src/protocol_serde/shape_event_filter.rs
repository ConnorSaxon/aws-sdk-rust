// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_event_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EventFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_arns {
        let mut array_2 = object.key("eventArns").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.event_type_codes {
        let mut array_5 = object.key("eventTypeCodes").start_array();
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
    if let Some(var_13) = &input.availability_zones {
        let mut array_14 = object.key("availabilityZones").start_array();
        for item_15 in var_13 {
             {
                array_14.value().string(item_15.as_str());
            }
        }
        array_14.finish();
    }
    if let Some(var_16) = &input.start_times {
        let mut array_17 = object.key("startTimes").start_array();
        for item_18 in var_16 {
             {
                #[allow(unused_mut)]
                let mut object_19 = array_17.value().start_object();
                crate::protocol_serde::shape_date_time_range::ser_date_time_range(&mut object_19, item_18)?;
                object_19.finish();
            }
        }
        array_17.finish();
    }
    if let Some(var_20) = &input.end_times {
        let mut array_21 = object.key("endTimes").start_array();
        for item_22 in var_20 {
             {
                #[allow(unused_mut)]
                let mut object_23 = array_21.value().start_object();
                crate::protocol_serde::shape_date_time_range::ser_date_time_range(&mut object_23, item_22)?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    if let Some(var_24) = &input.last_updated_times {
        let mut array_25 = object.key("lastUpdatedTimes").start_array();
        for item_26 in var_24 {
             {
                #[allow(unused_mut)]
                let mut object_27 = array_25.value().start_object();
                crate::protocol_serde::shape_date_time_range::ser_date_time_range(&mut object_27, item_26)?;
                object_27.finish();
            }
        }
        array_25.finish();
    }
    if let Some(var_28) = &input.entity_arns {
        let mut array_29 = object.key("entityArns").start_array();
        for item_30 in var_28 {
             {
                array_29.value().string(item_30.as_str());
            }
        }
        array_29.finish();
    }
    if let Some(var_31) = &input.entity_values {
        let mut array_32 = object.key("entityValues").start_array();
        for item_33 in var_31 {
             {
                array_32.value().string(item_33.as_str());
            }
        }
        array_32.finish();
    }
    if let Some(var_34) = &input.event_type_categories {
        let mut array_35 = object.key("eventTypeCategories").start_array();
        for item_36 in var_34 {
             {
                array_35.value().string(item_36.as_str());
            }
        }
        array_35.finish();
    }
    if let Some(var_37) = &input.tags {
        let mut array_38 = object.key("tags").start_array();
        for item_39 in var_37 {
             {
                #[allow(unused_mut)]
                let mut object_40 = array_38.value().start_object();
                for (key_41, value_42) in item_39 {
                     {
                        object_40.key(key_41.as_str()).string(value_42.as_str());
                    }
                }
                object_40.finish();
            }
        }
        array_38.finish();
    }
    if let Some(var_43) = &input.event_status_codes {
        let mut array_44 = object.key("eventStatusCodes").start_array();
        for item_45 in var_43 {
             {
                array_44.value().string(item_45.as_str());
            }
        }
        array_44.finish();
    }
    Ok(())
}

