// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_access_point_input_input(input: &crate::input::CreateAccessPointInput, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.bucket {
        let mut inner_writer = scope.start_el("Bucket").finish();
        inner_writer.data(
            var_1.as_str()
        );
    }
    if let Some(var_2) = &input.bucket_account_id {
        let mut inner_writer = scope.start_el("BucketAccountId").finish();
        inner_writer.data(
            var_2.as_str()
        );
    }
    if let Some(var_3) = &input.public_access_block_configuration {
        let inner_writer = scope.start_el("PublicAccessBlockConfiguration");
        crate::protocol_serde::shape_public_access_block_configuration::ser_public_access_block_configuration(var_3, inner_writer)?
    }
    if let Some(var_4) = &input.vpc_configuration {
        let inner_writer = scope.start_el("VpcConfiguration");
        crate::protocol_serde::shape_vpc_configuration::ser_vpc_configuration(var_4, inner_writer)?
    }
    scope.finish();
    Ok(())
}

