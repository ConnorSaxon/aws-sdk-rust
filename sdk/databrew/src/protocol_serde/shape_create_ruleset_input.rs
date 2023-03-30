// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_ruleset_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateRulesetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.rules {
        let mut array_4 = object.key("Rules").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_rule::ser_rule(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Tags").start_object();
        for (key_9, value_10) in var_7 {
             {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.target_arn {
        object.key("TargetArn").string(var_11.as_str());
    }
    Ok(())
}

