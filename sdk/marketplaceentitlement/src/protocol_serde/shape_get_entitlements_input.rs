// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_entitlements_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetEntitlementsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.product_code {
        object.key("ProductCode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filter {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Filter").start_object();
        for (key_4, value_5) in var_2 {
             {
                let mut array_6 = object_3.key(key_4.as_str()).start_array();
                for item_7 in value_5 {
                     {
                        array_6.value().string(item_7.as_str());
                    }
                }
                array_6.finish();
            }
        }
        object_3.finish();
    }
    if let Some(var_8) = &input.next_token {
        object.key("NextToken").string(var_8.as_str());
    }
    if let Some(var_9) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_9).into()));
    }
    Ok(())
}

