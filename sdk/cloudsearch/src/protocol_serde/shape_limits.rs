// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_limits(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::Limits, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::Limits::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("MaximumReplicationCount") /* MaximumReplicationCount com.amazonaws.cloudsearch#Limits$MaximumReplicationCount */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudsearch#MaximumReplicationCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_maximum_replication_count(var_1);
            }
            ,
            s if s.matches("MaximumPartitionCount") /* MaximumPartitionCount com.amazonaws.cloudsearch#Limits$MaximumPartitionCount */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.cloudsearch#MaximumPartitionCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_maximum_partition_count(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

