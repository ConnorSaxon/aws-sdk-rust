// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_apply_security_groups_to_load_balancer_input_input(input: &crate::input::ApplySecurityGroupsToLoadBalancerInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ApplySecurityGroupsToLoadBalancer", "2012-06-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LoadBalancerName");
    if let Some(var_2) = &input.load_balancer_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SecurityGroups");
    if let Some(var_4) = &input.security_groups {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

