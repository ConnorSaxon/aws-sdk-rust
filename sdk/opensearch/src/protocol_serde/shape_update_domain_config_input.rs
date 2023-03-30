// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_domain_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateDomainConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.access_policies {
        object.key("AccessPolicies").string(var_1.as_str());
    }
    if let Some(var_2) = &input.advanced_options {
        #[allow(unused_mut)]
        let mut object_3 = object.key("AdvancedOptions").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.advanced_security_options {
        #[allow(unused_mut)]
        let mut object_7 = object.key("AdvancedSecurityOptions").start_object();
        crate::protocol_serde::shape_advanced_security_options_input::ser_advanced_security_options_input(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.auto_tune_options {
        #[allow(unused_mut)]
        let mut object_9 = object.key("AutoTuneOptions").start_object();
        crate::protocol_serde::shape_auto_tune_options::ser_auto_tune_options(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.cluster_config {
        #[allow(unused_mut)]
        let mut object_11 = object.key("ClusterConfig").start_object();
        crate::protocol_serde::shape_cluster_config::ser_cluster_config(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.cognito_options {
        #[allow(unused_mut)]
        let mut object_13 = object.key("CognitoOptions").start_object();
        crate::protocol_serde::shape_cognito_options::ser_cognito_options(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.domain_endpoint_options {
        #[allow(unused_mut)]
        let mut object_15 = object.key("DomainEndpointOptions").start_object();
        crate::protocol_serde::shape_domain_endpoint_options::ser_domain_endpoint_options(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.dry_run {
        object.key("DryRun").boolean(*var_16);
    }
    if let Some(var_17) = &input.dry_run_mode {
        object.key("DryRunMode").string(var_17.as_str());
    }
    if let Some(var_18) = &input.ebs_options {
        #[allow(unused_mut)]
        let mut object_19 = object.key("EBSOptions").start_object();
        crate::protocol_serde::shape_ebs_options::ser_ebs_options(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.encryption_at_rest_options {
        #[allow(unused_mut)]
        let mut object_21 = object.key("EncryptionAtRestOptions").start_object();
        crate::protocol_serde::shape_encryption_at_rest_options::ser_encryption_at_rest_options(&mut object_21, var_20)?;
        object_21.finish();
    }
    if let Some(var_22) = &input.log_publishing_options {
        #[allow(unused_mut)]
        let mut object_23 = object.key("LogPublishingOptions").start_object();
        for (key_24, value_25) in var_22 {
             {
                #[allow(unused_mut)]
                let mut object_26 = object_23.key(key_24.as_str()).start_object();
                crate::protocol_serde::shape_log_publishing_option::ser_log_publishing_option(&mut object_26, value_25)?;
                object_26.finish();
            }
        }
        object_23.finish();
    }
    if let Some(var_27) = &input.node_to_node_encryption_options {
        #[allow(unused_mut)]
        let mut object_28 = object.key("NodeToNodeEncryptionOptions").start_object();
        crate::protocol_serde::shape_node_to_node_encryption_options::ser_node_to_node_encryption_options(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.snapshot_options {
        #[allow(unused_mut)]
        let mut object_30 = object.key("SnapshotOptions").start_object();
        crate::protocol_serde::shape_snapshot_options::ser_snapshot_options(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.vpc_options {
        #[allow(unused_mut)]
        let mut object_32 = object.key("VPCOptions").start_object();
        crate::protocol_serde::shape_vpc_options::ser_vpc_options(&mut object_32, var_31)?;
        object_32.finish();
    }
    Ok(())
}

