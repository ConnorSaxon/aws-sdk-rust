// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_sampling_rule_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSamplingRuleInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.sampling_rule {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SamplingRule").start_object();
        crate::protocol_serde::shape_sampling_rule::ser_sampling_rule(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.tags {
        let mut array_4 = object.key("Tags").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}

