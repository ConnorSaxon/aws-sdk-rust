// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_regex_match_set_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateRegexMatchSetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.regex_match_set_id {
        object.key("RegexMatchSetId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.updates {
        let mut array_3 = object.key("Updates").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_regex_match_set_update::ser_regex_match_set_update(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.change_token {
        object.key("ChangeToken").string(var_6.as_str());
    }
    Ok(())
}

