// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`CreateFilter`](crate::client::fluent_builders::CreateFilter) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`detector_id(impl Into<String>)`](crate::client::fluent_builders::CreateFilter::detector_id) / [`set_detector_id(Option<String>)`](crate::client::fluent_builders::CreateFilter::set_detector_id): <p>The ID of the detector belonging to the GuardDuty account that you want to create a filter for.</p>
    ///   - [`name(impl Into<String>)`](crate::client::fluent_builders::CreateFilter::name) / [`set_name(Option<String>)`](crate::client::fluent_builders::CreateFilter::set_name): <p>The name of the filter. Valid characters include period (.), underscore (_), dash (-), and alphanumeric characters. A whitespace is considered to be an invalid character.</p>
    ///   - [`description(impl Into<String>)`](crate::client::fluent_builders::CreateFilter::description) / [`set_description(Option<String>)`](crate::client::fluent_builders::CreateFilter::set_description): <p>The description of the filter. Valid special characters include period (.), underscore (_), dash (-), and whitespace. The new line character is considered to be an invalid input for description.</p>
    ///   - [`action(FilterAction)`](crate::client::fluent_builders::CreateFilter::action) / [`set_action(Option<FilterAction>)`](crate::client::fluent_builders::CreateFilter::set_action): <p>Specifies the action that is to be applied to the findings that match the filter.</p>
    ///   - [`rank(i32)`](crate::client::fluent_builders::CreateFilter::rank) / [`set_rank(i32)`](crate::client::fluent_builders::CreateFilter::set_rank): <p>Specifies the position of the filter in the list of current filters. Also specifies the order in which this filter is applied to the findings.</p>
    ///   - [`finding_criteria(FindingCriteria)`](crate::client::fluent_builders::CreateFilter::finding_criteria) / [`set_finding_criteria(Option<FindingCriteria>)`](crate::client::fluent_builders::CreateFilter::set_finding_criteria): <p>Represents the criteria to be used in the filter for querying findings.</p>  <p>You can only use the following attributes to query findings:</p>  <ul>   <li> <p>accountId</p> </li>   <li> <p>region</p> </li>   <li> <p>confidence</p> </li>   <li> <p>id</p> </li>   <li> <p>resource.accessKeyDetails.accessKeyId</p> </li>   <li> <p>resource.accessKeyDetails.principalId</p> </li>   <li> <p>resource.accessKeyDetails.userName</p> </li>   <li> <p>resource.accessKeyDetails.userType</p> </li>   <li> <p>resource.instanceDetails.iamInstanceProfile.id</p> </li>   <li> <p>resource.instanceDetails.imageId</p> </li>   <li> <p>resource.instanceDetails.instanceId</p> </li>   <li> <p>resource.instanceDetails.outpostArn</p> </li>   <li> <p>resource.instanceDetails.networkInterfaces.ipv6Addresses</p> </li>   <li> <p>resource.instanceDetails.networkInterfaces.privateIpAddresses.privateIpAddress</p> </li>   <li> <p>resource.instanceDetails.networkInterfaces.publicDnsName</p> </li>   <li> <p>resource.instanceDetails.networkInterfaces.publicIp</p> </li>   <li> <p>resource.instanceDetails.networkInterfaces.securityGroups.groupId</p> </li>   <li> <p>resource.instanceDetails.networkInterfaces.securityGroups.groupName</p> </li>   <li> <p>resource.instanceDetails.networkInterfaces.subnetId</p> </li>   <li> <p>resource.instanceDetails.networkInterfaces.vpcId</p> </li>   <li> <p>resource.instanceDetails.tags.key</p> </li>   <li> <p>resource.instanceDetails.tags.value</p> </li>   <li> <p>resource.resourceType</p> </li>   <li> <p>service.action.actionType</p> </li>   <li> <p>service.action.awsApiCallAction.api</p> </li>   <li> <p>service.action.awsApiCallAction.callerType</p> </li>   <li> <p>service.action.awsApiCallAction.errorCode</p> </li>   <li> <p>service.action.awsApiCallAction.userAgent</p> </li>   <li> <p>service.action.awsApiCallAction.remoteIpDetails.city.cityName</p> </li>   <li> <p>service.action.awsApiCallAction.remoteIpDetails.country.countryName</p> </li>   <li> <p>service.action.awsApiCallAction.remoteIpDetails.ipAddressV4</p> </li>   <li> <p>service.action.awsApiCallAction.remoteIpDetails.organization.asn</p> </li>   <li> <p>service.action.awsApiCallAction.remoteIpDetails.organization.asnOrg</p> </li>   <li> <p>service.action.awsApiCallAction.serviceName</p> </li>   <li> <p>service.action.dnsRequestAction.domain</p> </li>   <li> <p>service.action.networkConnectionAction.blocked</p> </li>   <li> <p>service.action.networkConnectionAction.connectionDirection</p> </li>   <li> <p>service.action.networkConnectionAction.localPortDetails.port</p> </li>   <li> <p>service.action.networkConnectionAction.protocol</p> </li>   <li> <p>service.action.networkConnectionAction.localIpDetails.ipAddressV4</p> </li>   <li> <p>service.action.networkConnectionAction.remoteIpDetails.city.cityName</p> </li>   <li> <p>service.action.networkConnectionAction.remoteIpDetails.country.countryName</p> </li>   <li> <p>service.action.networkConnectionAction.remoteIpDetails.ipAddressV4</p> </li>   <li> <p>service.action.networkConnectionAction.remoteIpDetails.organization.asn</p> </li>   <li> <p>service.action.networkConnectionAction.remoteIpDetails.organization.asnOrg</p> </li>   <li> <p>service.action.networkConnectionAction.remotePortDetails.port</p> </li>   <li> <p>service.additionalInfo.threatListName</p> </li>   <li> <p>resource.s3BucketDetails.publicAccess.effectivePermissions</p> </li>   <li> <p>resource.s3BucketDetails.name</p> </li>   <li> <p>resource.s3BucketDetails.tags.key</p> </li>   <li> <p>resource.s3BucketDetails.tags.value</p> </li>   <li> <p>resource.s3BucketDetails.type</p> </li>   <li> <p>service.archived</p> <p>When this attribute is set to TRUE, only archived findings are listed. When it's set to FALSE, only unarchived findings are listed. When this attribute is not set, all existing findings are listed.</p> </li>   <li> <p>service.resourceRole</p> </li>   <li> <p>severity</p> </li>   <li> <p>type</p> </li>   <li> <p>updatedAt</p> <p>Type: ISO 8601 string format: YYYY-MM-DDTHH:MM:SS.SSSZ or YYYY-MM-DDTHH:MM:SSZ depending on whether the value contains milliseconds.</p> </li>  </ul>
    ///   - [`client_token(impl Into<String>)`](crate::client::fluent_builders::CreateFilter::client_token) / [`set_client_token(Option<String>)`](crate::client::fluent_builders::CreateFilter::set_client_token): <p>The idempotency token for the create request.</p>
    ///   - [`tags(HashMap<String, String>)`](crate::client::fluent_builders::CreateFilter::tags) / [`set_tags(Option<HashMap<String, String>>)`](crate::client::fluent_builders::CreateFilter::set_tags): <p>The tags to be added to a new filter resource.</p>
                            /// - On success, responds with [`CreateFilterOutput`](crate::output::CreateFilterOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::output::CreateFilterOutput::name): <p>The name of the successfully created filter.</p>
                            /// - On failure, responds with [`SdkError<CreateFilterError>`](crate::error::CreateFilterError)
    pub fn create_filter(&self) -> crate::client::fluent_builders::CreateFilter {
                                crate::client::fluent_builders::CreateFilter::new(self.handle.clone())
                            }
}

