// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_stack_set_input_input(input: &crate::input::CreateStackSetInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateStackSet", "2010-05-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("StackSetName");
    if let Some(var_2) = &input.stack_set_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Description");
    if let Some(var_4) = &input.description {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TemplateBody");
    if let Some(var_6) = &input.template_body {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("TemplateURL");
    if let Some(var_8) = &input.template_url {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("StackId");
    if let Some(var_10) = &input.stack_id {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Parameters");
    if let Some(var_12) = &input.parameters {
        let mut list_14 = scope_11.start_list(false, None);
        for item_13 in var_12 {
            #[allow(unused_mut)]
            let mut entry_15 = list_14.entry();
            crate::protocol_serde::shape_parameter::ser_parameter(entry_15, item_13)?;
        }
        list_14.finish();
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("Capabilities");
    if let Some(var_17) = &input.capabilities {
        let mut list_19 = scope_16.start_list(false, None);
        for item_18 in var_17 {
            #[allow(unused_mut)]
            let mut entry_20 = list_19.entry();
            entry_20.string(item_18.as_str());
        }
        list_19.finish();
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("Tags");
    if let Some(var_22) = &input.tags {
        let mut list_24 = scope_21.start_list(false, None);
        for item_23 in var_22 {
            #[allow(unused_mut)]
            let mut entry_25 = list_24.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_25, item_23)?;
        }
        list_24.finish();
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("AdministrationRoleARN");
    if let Some(var_27) = &input.administration_role_arn {
        scope_26.string(var_27);
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("ExecutionRoleName");
    if let Some(var_29) = &input.execution_role_name {
        scope_28.string(var_29);
    }
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("PermissionModel");
    if let Some(var_31) = &input.permission_model {
        scope_30.string(var_31.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_32 = writer.prefix("AutoDeployment");
    if let Some(var_33) = &input.auto_deployment {
        crate::protocol_serde::shape_auto_deployment::ser_auto_deployment(scope_32, var_33)?;
    }
    #[allow(unused_mut)]
    let mut scope_34 = writer.prefix("CallAs");
    if let Some(var_35) = &input.call_as {
        scope_34.string(var_35.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_36 = writer.prefix("ClientRequestToken");
    if let Some(var_37) = &input.client_request_token {
        scope_36.string(var_37);
    }
    #[allow(unused_mut)]
    let mut scope_38 = writer.prefix("ManagedExecution");
    if let Some(var_39) = &input.managed_execution {
        crate::protocol_serde::shape_managed_execution::ser_managed_execution(scope_38, var_39)?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

