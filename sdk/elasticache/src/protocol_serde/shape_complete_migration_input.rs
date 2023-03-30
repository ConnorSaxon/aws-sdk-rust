// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_complete_migration_input_input(input: &crate::input::CompleteMigrationInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CompleteMigration", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ReplicationGroupId");
    if let Some(var_2) = &input.replication_group_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Force");
    if input.force {
        scope_3.boolean(input.force);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

