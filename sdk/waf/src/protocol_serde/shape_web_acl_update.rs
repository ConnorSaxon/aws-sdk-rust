// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_web_acl_update(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::WebAclUpdate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("Action").string(var_1.as_str());
    }
    if let Some(var_2) = &input.activated_rule {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ActivatedRule").start_object();
        crate::protocol_serde::shape_activated_rule::ser_activated_rule(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

