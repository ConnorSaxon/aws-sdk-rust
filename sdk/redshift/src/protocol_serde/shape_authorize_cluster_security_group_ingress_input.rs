// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_authorize_cluster_security_group_ingress_input_input(input: &crate::input::AuthorizeClusterSecurityGroupIngressInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "AuthorizeClusterSecurityGroupIngress", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClusterSecurityGroupName");
    if let Some(var_2) = &input.cluster_security_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("CIDRIP");
    if let Some(var_4) = &input.cidrip {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("EC2SecurityGroupName");
    if let Some(var_6) = &input.ec2_security_group_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("EC2SecurityGroupOwnerId");
    if let Some(var_8) = &input.ec2_security_group_owner_id {
        scope_7.string(var_8);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

