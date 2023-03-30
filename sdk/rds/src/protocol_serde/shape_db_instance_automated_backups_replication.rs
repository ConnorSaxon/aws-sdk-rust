// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_db_instance_automated_backups_replication(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DbInstanceAutomatedBackupsReplication, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DbInstanceAutomatedBackupsReplication::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DBInstanceAutomatedBackupsArn") /* DBInstanceAutomatedBackupsArn com.amazonaws.rds#DBInstanceAutomatedBackupsReplication$DBInstanceAutomatedBackupsArn */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_instance_automated_backups_arn(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

