// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_ip_routes_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddIpRoutesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.directory_id {
        object.key("DirectoryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ip_routes {
        let mut array_3 = object.key("IpRoutes").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_ip_route::ser_ip_route(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if input.update_security_group_for_directory_controllers {
        object.key("UpdateSecurityGroupForDirectoryControllers").boolean(input.update_security_group_for_directory_controllers);
    }
    Ok(())
}

