// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_instance_count(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::InstanceCount, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::InstanceCount::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceCount") /* InstanceCount com.amazonaws.ec2#InstanceCount$InstanceCount */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_instance_count(var_1);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#InstanceCount$State */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::model::ListingState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::ListingState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

