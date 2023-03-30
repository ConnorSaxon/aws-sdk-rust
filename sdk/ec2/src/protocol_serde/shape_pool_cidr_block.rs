// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_pool_cidr_block(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::PoolCidrBlock, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::PoolCidrBlock::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("poolCidrBlock") /* Cidr com.amazonaws.ec2#PoolCidrBlock$Cidr */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cidr(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

