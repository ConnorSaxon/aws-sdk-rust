// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_inclusion_protection_filters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InclusionProtectionFilters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_arns {
        let mut array_2 = object.key("ResourceArns").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.protection_names {
        let mut array_5 = object.key("ProtectionNames").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.resource_types {
        let mut array_8 = object.key("ResourceTypes").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    Ok(())
}

