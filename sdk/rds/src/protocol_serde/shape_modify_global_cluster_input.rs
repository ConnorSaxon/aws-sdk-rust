// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_global_cluster_input_input(input: &crate::input::ModifyGlobalClusterInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ModifyGlobalCluster", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("GlobalClusterIdentifier");
    if let Some(var_2) = &input.global_cluster_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("NewGlobalClusterIdentifier");
    if let Some(var_4) = &input.new_global_cluster_identifier {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DeletionProtection");
    if let Some(var_6) = &input.deletion_protection {
        scope_5.boolean(*var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("EngineVersion");
    if let Some(var_8) = &input.engine_version {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("AllowMajorVersionUpgrade");
    if let Some(var_10) = &input.allow_major_version_upgrade {
        scope_9.boolean(*var_10);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

