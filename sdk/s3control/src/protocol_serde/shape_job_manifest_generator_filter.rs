// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_job_manifest_generator_filter(input: &crate::model::JobManifestGeneratorFilter, writer: aws_smithy_xml::encode::ElWriter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.eligible_for_replication {
        let mut inner_writer = scope.start_el("EligibleForReplication").finish();
        inner_writer.data(
            aws_smithy_types::primitive::Encoder::from(*var_1).encode()
        );
    }
    if let Some(var_2) = &input.created_after {
        let mut inner_writer = scope.start_el("CreatedAfter").finish();
        inner_writer.data(
            var_2.fmt(aws_smithy_types::date_time::Format::DateTime)?.as_ref()
        );
    }
    if let Some(var_3) = &input.created_before {
        let mut inner_writer = scope.start_el("CreatedBefore").finish();
        inner_writer.data(
            var_3.fmt(aws_smithy_types::date_time::Format::DateTime)?.as_ref()
        );
    }
    if let Some(var_4) = &input.object_replication_statuses {
        let mut inner_writer = scope.start_el("ObjectReplicationStatuses").finish();
        for list_item_5 in var_4 {
             {
                let mut inner_writer = inner_writer.start_el("member").finish();
                inner_writer.data(
                    list_item_5.as_str()
                );
            }
        }
    }
    scope.finish();
    Ok(())
}

pub fn de_job_manifest_generator_filter(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::JobManifestGeneratorFilter, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::JobManifestGeneratorFilter::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("EligibleForReplication") /* EligibleForReplication com.amazonaws.s3control#JobManifestGeneratorFilter$EligibleForReplication */ =>  {
                let var_6 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3control#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_eligible_for_replication(var_6);
            }
            ,
            s if s.matches("CreatedAfter") /* CreatedAfter com.amazonaws.s3control#JobManifestGeneratorFilter$CreatedAfter */ =>  {
                let var_7 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3control#ObjectCreationTime`)"))
                        ?
                    )
                ;
                builder = builder.set_created_after(var_7);
            }
            ,
            s if s.matches("CreatedBefore") /* CreatedBefore com.amazonaws.s3control#JobManifestGeneratorFilter$CreatedBefore */ =>  {
                let var_8 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3control#ObjectCreationTime`)"))
                        ?
                    )
                ;
                builder = builder.set_created_before(var_8);
            }
            ,
            s if s.matches("ObjectReplicationStatuses") /* ObjectReplicationStatuses com.amazonaws.s3control#JobManifestGeneratorFilter$ObjectReplicationStatuses */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_replication_status_filter_list::de_replication_status_filter_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_object_replication_statuses(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

