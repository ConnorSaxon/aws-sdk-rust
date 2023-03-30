// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_ec2_vpc_peering_connection_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsEc2VpcPeeringConnectionDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.accepter_vpc_info {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AccepterVpcInfo").start_object();
        crate::protocol_serde::shape_aws_ec2_vpc_peering_connection_vpc_info_details::ser_aws_ec2_vpc_peering_connection_vpc_info_details(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.expiration_time {
        object.key("ExpirationTime").string(var_3.as_str());
    }
    if let Some(var_4) = &input.requester_vpc_info {
        #[allow(unused_mut)]
        let mut object_5 = object.key("RequesterVpcInfo").start_object();
        crate::protocol_serde::shape_aws_ec2_vpc_peering_connection_vpc_info_details::ser_aws_ec2_vpc_peering_connection_vpc_info_details(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.status {
        #[allow(unused_mut)]
        let mut object_7 = object.key("Status").start_object();
        crate::protocol_serde::shape_aws_ec2_vpc_peering_connection_status_details::ser_aws_ec2_vpc_peering_connection_status_details(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.vpc_peering_connection_id {
        object.key("VpcPeeringConnectionId").string(var_8.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_ec2_vpc_peering_connection_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsEc2VpcPeeringConnectionDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_ec2_vpc_peering_connection_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AccepterVpcInfo" => {
                                builder = builder.set_accepter_vpc_info(
                                    crate::protocol_serde::shape_aws_ec2_vpc_peering_connection_vpc_info_details::de_aws_ec2_vpc_peering_connection_vpc_info_details(tokens)?
                                );
                            }
                            "ExpirationTime" => {
                                builder = builder.set_expiration_time(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RequesterVpcInfo" => {
                                builder = builder.set_requester_vpc_info(
                                    crate::protocol_serde::shape_aws_ec2_vpc_peering_connection_vpc_info_details::de_aws_ec2_vpc_peering_connection_vpc_info_details(tokens)?
                                );
                            }
                            "Status" => {
                                builder = builder.set_status(
                                    crate::protocol_serde::shape_aws_ec2_vpc_peering_connection_status_details::de_aws_ec2_vpc_peering_connection_status_details(tokens)?
                                );
                            }
                            "VpcPeeringConnectionId" => {
                                builder = builder.set_vpc_peering_connection_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

