// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_access_point_policy_input_input(input: &crate::input::PutAccessPointPolicyInput, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.policy {
        let mut inner_writer = scope.start_el("Policy").finish();
        inner_writer.data(
            var_1.as_str()
        );
    }
    scope.finish();
    Ok(())
}

