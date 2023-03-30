// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_custom_data_identifier_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateCustomDataIdentifierInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ignore_words {
        let mut array_4 = object.key("ignoreWords").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.keywords {
        let mut array_7 = object.key("keywords").start_array();
        for item_8 in var_6 {
             {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if input.maximum_match_distance != 0 {
        object.key("maximumMatchDistance").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.maximum_match_distance).into()));
    }
    if let Some(var_9) = &input.name {
        object.key("name").string(var_9.as_str());
    }
    if let Some(var_10) = &input.regex {
        object.key("regex").string(var_10.as_str());
    }
    if let Some(var_11) = &input.severity_levels {
        let mut array_12 = object.key("severityLevels").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_severity_level::ser_severity_level(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.tags {
        #[allow(unused_mut)]
        let mut object_16 = object.key("tags").start_object();
        for (key_17, value_18) in var_15 {
             {
                object_16.key(key_17.as_str()).string(value_18.as_str());
            }
        }
        object_16.finish();
    }
    Ok(())
}

