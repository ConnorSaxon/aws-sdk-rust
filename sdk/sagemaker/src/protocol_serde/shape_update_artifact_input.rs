// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_artifact_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateArtifactInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.artifact_arn {
        object.key("ArtifactArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.artifact_name {
        object.key("ArtifactName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.properties {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Properties").start_object();
        for (key_5, value_6) in var_3 {
             {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.properties_to_remove {
        let mut array_8 = object.key("PropertiesToRemove").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    Ok(())
}

