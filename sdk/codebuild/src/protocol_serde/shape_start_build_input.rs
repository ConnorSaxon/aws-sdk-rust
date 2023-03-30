// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_build_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartBuildInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.project_name {
        object.key("projectName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.secondary_sources_override {
        let mut array_3 = object.key("secondarySourcesOverride").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_project_source::ser_project_source(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.secondary_sources_version_override {
        let mut array_7 = object.key("secondarySourcesVersionOverride").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_project_source_version::ser_project_source_version(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.source_version {
        object.key("sourceVersion").string(var_10.as_str());
    }
    if let Some(var_11) = &input.artifacts_override {
        #[allow(unused_mut)]
        let mut object_12 = object.key("artifactsOverride").start_object();
        crate::protocol_serde::shape_project_artifacts::ser_project_artifacts(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.secondary_artifacts_override {
        let mut array_14 = object.key("secondaryArtifactsOverride").start_array();
        for item_15 in var_13 {
             {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_project_artifacts::ser_project_artifacts(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.environment_variables_override {
        let mut array_18 = object.key("environmentVariablesOverride").start_array();
        for item_19 in var_17 {
             {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_environment_variable::ser_environment_variable(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    if let Some(var_21) = &input.source_type_override {
        object.key("sourceTypeOverride").string(var_21.as_str());
    }
    if let Some(var_22) = &input.source_location_override {
        object.key("sourceLocationOverride").string(var_22.as_str());
    }
    if let Some(var_23) = &input.source_auth_override {
        #[allow(unused_mut)]
        let mut object_24 = object.key("sourceAuthOverride").start_object();
        crate::protocol_serde::shape_source_auth::ser_source_auth(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.git_clone_depth_override {
        object.key("gitCloneDepthOverride").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_25).into()));
    }
    if let Some(var_26) = &input.git_submodules_config_override {
        #[allow(unused_mut)]
        let mut object_27 = object.key("gitSubmodulesConfigOverride").start_object();
        crate::protocol_serde::shape_git_submodules_config::ser_git_submodules_config(&mut object_27, var_26)?;
        object_27.finish();
    }
    if let Some(var_28) = &input.buildspec_override {
        object.key("buildspecOverride").string(var_28.as_str());
    }
    if let Some(var_29) = &input.insecure_ssl_override {
        object.key("insecureSslOverride").boolean(*var_29);
    }
    if let Some(var_30) = &input.report_build_status_override {
        object.key("reportBuildStatusOverride").boolean(*var_30);
    }
    if let Some(var_31) = &input.build_status_config_override {
        #[allow(unused_mut)]
        let mut object_32 = object.key("buildStatusConfigOverride").start_object();
        crate::protocol_serde::shape_build_status_config::ser_build_status_config(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.environment_type_override {
        object.key("environmentTypeOverride").string(var_33.as_str());
    }
    if let Some(var_34) = &input.image_override {
        object.key("imageOverride").string(var_34.as_str());
    }
    if let Some(var_35) = &input.compute_type_override {
        object.key("computeTypeOverride").string(var_35.as_str());
    }
    if let Some(var_36) = &input.certificate_override {
        object.key("certificateOverride").string(var_36.as_str());
    }
    if let Some(var_37) = &input.cache_override {
        #[allow(unused_mut)]
        let mut object_38 = object.key("cacheOverride").start_object();
        crate::protocol_serde::shape_project_cache::ser_project_cache(&mut object_38, var_37)?;
        object_38.finish();
    }
    if let Some(var_39) = &input.service_role_override {
        object.key("serviceRoleOverride").string(var_39.as_str());
    }
    if let Some(var_40) = &input.privileged_mode_override {
        object.key("privilegedModeOverride").boolean(*var_40);
    }
    if let Some(var_41) = &input.timeout_in_minutes_override {
        object.key("timeoutInMinutesOverride").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_41).into()));
    }
    if let Some(var_42) = &input.queued_timeout_in_minutes_override {
        object.key("queuedTimeoutInMinutesOverride").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_42).into()));
    }
    if let Some(var_43) = &input.encryption_key_override {
        object.key("encryptionKeyOverride").string(var_43.as_str());
    }
    if let Some(var_44) = &input.idempotency_token {
        object.key("idempotencyToken").string(var_44.as_str());
    }
    if let Some(var_45) = &input.logs_config_override {
        #[allow(unused_mut)]
        let mut object_46 = object.key("logsConfigOverride").start_object();
        crate::protocol_serde::shape_logs_config::ser_logs_config(&mut object_46, var_45)?;
        object_46.finish();
    }
    if let Some(var_47) = &input.registry_credential_override {
        #[allow(unused_mut)]
        let mut object_48 = object.key("registryCredentialOverride").start_object();
        crate::protocol_serde::shape_registry_credential::ser_registry_credential(&mut object_48, var_47)?;
        object_48.finish();
    }
    if let Some(var_49) = &input.image_pull_credentials_type_override {
        object.key("imagePullCredentialsTypeOverride").string(var_49.as_str());
    }
    if let Some(var_50) = &input.debug_session_enabled {
        object.key("debugSessionEnabled").boolean(*var_50);
    }
    Ok(())
}

