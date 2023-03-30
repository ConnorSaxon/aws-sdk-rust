// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_ipam_pool_allocation(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::IpamPoolAllocation, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::IpamPoolAllocation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("cidr") /* Cidr com.amazonaws.ec2#IpamPoolAllocation$Cidr */ =>  {
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
            s if s.matches("ipamPoolAllocationId") /* IpamPoolAllocationId com.amazonaws.ec2#IpamPoolAllocation$IpamPoolAllocationId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_pool_allocation_id(var_2);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#IpamPoolAllocation$Description */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_3);
            }
            ,
            s if s.matches("resourceId") /* ResourceId com.amazonaws.ec2#IpamPoolAllocation$ResourceId */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_id(var_4);
            }
            ,
            s if s.matches("resourceType") /* ResourceType com.amazonaws.ec2#IpamPoolAllocation$ResourceType */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::model::IpamPoolAllocationResourceType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::IpamPoolAllocationResourceType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_type(var_5);
            }
            ,
            s if s.matches("resourceRegion") /* ResourceRegion com.amazonaws.ec2#IpamPoolAllocation$ResourceRegion */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_region(var_6);
            }
            ,
            s if s.matches("resourceOwner") /* ResourceOwner com.amazonaws.ec2#IpamPoolAllocation$ResourceOwner */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_owner(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

