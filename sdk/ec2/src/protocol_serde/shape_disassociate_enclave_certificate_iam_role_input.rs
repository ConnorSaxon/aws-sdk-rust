// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_enclave_certificate_iam_role_input_input(input: &crate::input::DisassociateEnclaveCertificateIamRoleInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DisassociateEnclaveCertificateIamRole", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CertificateArn");
    if let Some(var_2) = &input.certificate_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("RoleArn");
    if let Some(var_4) = &input.role_arn {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DryRun");
    if let Some(var_6) = &input.dry_run {
        scope_5.boolean(*var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

