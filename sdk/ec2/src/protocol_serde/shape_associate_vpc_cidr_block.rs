// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_vpc_cidr_block_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AssociateVpcCidrBlockOutput, crate::error::AssociateVpcCidrBlockError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AssociateVpcCidrBlockError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::AssociateVpcCidrBlockError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_vpc_cidr_block_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AssociateVpcCidrBlockOutput, crate::error::AssociateVpcCidrBlockError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::associate_vpc_cidr_block_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_associate_vpc_cidr_block::de_associate_vpc_cidr_block(response.body().as_ref(), output).map_err(crate::error::AssociateVpcCidrBlockError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_associate_vpc_cidr_block(inp: &[u8], mut builder: crate::output::associate_vpc_cidr_block_output::Builder) -> Result<crate::output::associate_vpc_cidr_block_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("AssociateVpcCidrBlockResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected AssociateVpcCidrBlockResponse got {:?}", start_el)))
                    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ipv6CidrBlockAssociation") /* Ipv6CidrBlockAssociation com.amazonaws.ec2.synthetic#AssociateVpcCidrBlockOutput$Ipv6CidrBlockAssociation */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_vpc_ipv6_cidr_block_association::de_vpc_ipv6_cidr_block_association(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv6_cidr_block_association(var_1);
            }
            ,
            s if s.matches("cidrBlockAssociation") /* CidrBlockAssociation com.amazonaws.ec2.synthetic#AssociateVpcCidrBlockOutput$CidrBlockAssociation */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_vpc_cidr_block_association::de_vpc_cidr_block_association(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cidr_block_association(var_2);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2.synthetic#AssociateVpcCidrBlockOutput$VpcId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

