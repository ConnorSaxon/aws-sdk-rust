// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.policy {
        #[allow(unused_mut)]
        let mut object_2 = object.key("policy").start_object();
        crate::protocol_serde::shape_policy::ser_policy(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

