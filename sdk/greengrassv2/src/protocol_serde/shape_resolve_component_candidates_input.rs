// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resolve_component_candidates_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ResolveComponentCandidatesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.component_candidates {
        let mut array_2 = object.key("componentCandidates").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_component_candidate::ser_component_candidate(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.platform {
        #[allow(unused_mut)]
        let mut object_6 = object.key("platform").start_object();
        crate::protocol_serde::shape_component_platform::ser_component_platform(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

