// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_scan_ec2_instance_with_findings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ScanEc2InstanceWithFindings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.ebs_volumes {
        object.key("ebsVolumes").boolean(input.ebs_volumes);
    }
    Ok(())
}

