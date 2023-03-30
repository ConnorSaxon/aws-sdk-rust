// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_package_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdatePackageInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.commit_message {
        object.key("CommitMessage").string(var_1.as_str());
    }
    if let Some(var_2) = &input.package_description {
        object.key("PackageDescription").string(var_2.as_str());
    }
    if let Some(var_3) = &input.package_id {
        object.key("PackageID").string(var_3.as_str());
    }
    if let Some(var_4) = &input.package_source {
        #[allow(unused_mut)]
        let mut object_5 = object.key("PackageSource").start_object();
        crate::protocol_serde::shape_package_source::ser_package_source(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

