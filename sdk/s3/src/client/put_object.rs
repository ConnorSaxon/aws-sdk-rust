// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`PutObject`](crate::client::fluent_builders::PutObject) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`acl(ObjectCannedAcl)`](crate::client::fluent_builders::PutObject::acl) / [`set_acl(Option<ObjectCannedAcl>)`](crate::client::fluent_builders::PutObject::set_acl): <p>The canned ACL to apply to the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned ACL</a>.</p>  <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///   - [`body(ByteStream)`](crate::client::fluent_builders::PutObject::body) / [`set_body(ByteStream)`](crate::client::fluent_builders::PutObject::set_body): <p>Object data.</p>
    ///   - [`bucket(impl Into<String>)`](crate::client::fluent_builders::PutObject::bucket) / [`set_bucket(Option<String>)`](crate::client::fluent_builders::PutObject::set_bucket): <p>The bucket name to which the PUT action was initiated. </p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`cache_control(impl Into<String>)`](crate::client::fluent_builders::PutObject::cache_control) / [`set_cache_control(Option<String>)`](crate::client::fluent_builders::PutObject::set_cache_control): <p> Can be used to specify caching behavior along the request/reply chain. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.9</a>.</p>
    ///   - [`content_disposition(impl Into<String>)`](crate::client::fluent_builders::PutObject::content_disposition) / [`set_content_disposition(Option<String>)`](crate::client::fluent_builders::PutObject::set_content_disposition): <p>Specifies presentational information for the object. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1">http://www.w3.org/Protocols/rfc2616/rfc2616-sec19.html#sec19.5.1</a>.</p>
    ///   - [`content_encoding(impl Into<String>)`](crate::client::fluent_builders::PutObject::content_encoding) / [`set_content_encoding(Option<String>)`](crate::client::fluent_builders::PutObject::set_content_encoding): <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.11</a>.</p>
    ///   - [`content_language(impl Into<String>)`](crate::client::fluent_builders::PutObject::content_language) / [`set_content_language(Option<String>)`](crate::client::fluent_builders::PutObject::set_content_language): <p>The language the content is in.</p>
    ///   - [`content_length(i64)`](crate::client::fluent_builders::PutObject::content_length) / [`set_content_length(i64)`](crate::client::fluent_builders::PutObject::set_content_length): <p>Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.13">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.13</a>.</p>
    ///   - [`content_md5(impl Into<String>)`](crate::client::fluent_builders::PutObject::content_md5) / [`set_content_md5(Option<String>)`](crate::client::fluent_builders::PutObject::set_content_md5): <p>The base64-encoded 128-bit MD5 digest of the message (without the headers) according to RFC 1864. This header can be used as a message integrity check to verify that the data is the same data that was originally sent. Although it is optional, we recommend using the Content-MD5 mechanism as an end-to-end integrity check. For more information about REST request authentication, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/RESTAuthentication.html">REST Authentication</a>.</p>
    ///   - [`content_type(impl Into<String>)`](crate::client::fluent_builders::PutObject::content_type) / [`set_content_type(Option<String>)`](crate::client::fluent_builders::PutObject::set_content_type): <p>A standard MIME type describing the format of the contents. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.17</a>.</p>
    ///   - [`checksum_algorithm(ChecksumAlgorithm)`](crate::client::fluent_builders::PutObject::checksum_algorithm) / [`set_checksum_algorithm(Option<ChecksumAlgorithm>)`](crate::client::fluent_builders::PutObject::set_checksum_algorithm): <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    ///   - [`checksum_crc32(impl Into<String>)`](crate::client::fluent_builders::PutObject::checksum_crc32) / [`set_checksum_crc32(Option<String>)`](crate::client::fluent_builders::PutObject::set_checksum_crc32): <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_crc32_c(impl Into<String>)`](crate::client::fluent_builders::PutObject::checksum_crc32_c) / [`set_checksum_crc32_c(Option<String>)`](crate::client::fluent_builders::PutObject::set_checksum_crc32_c): <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha1(impl Into<String>)`](crate::client::fluent_builders::PutObject::checksum_sha1) / [`set_checksum_sha1(Option<String>)`](crate::client::fluent_builders::PutObject::set_checksum_sha1): <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha256(impl Into<String>)`](crate::client::fluent_builders::PutObject::checksum_sha256) / [`set_checksum_sha256(Option<String>)`](crate::client::fluent_builders::PutObject::set_checksum_sha256): <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`expires(DateTime)`](crate::client::fluent_builders::PutObject::expires) / [`set_expires(Option<DateTime>)`](crate::client::fluent_builders::PutObject::set_expires): <p>The date and time at which the object is no longer cacheable. For more information, see <a href="http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.21">http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.21</a>.</p>
    ///   - [`grant_full_control(impl Into<String>)`](crate::client::fluent_builders::PutObject::grant_full_control) / [`set_grant_full_control(Option<String>)`](crate::client::fluent_builders::PutObject::set_grant_full_control): <p>Gives the grantee READ, READ_ACP, and WRITE_ACP permissions on the object.</p>  <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///   - [`grant_read(impl Into<String>)`](crate::client::fluent_builders::PutObject::grant_read) / [`set_grant_read(Option<String>)`](crate::client::fluent_builders::PutObject::set_grant_read): <p>Allows grantee to read the object data and its metadata.</p>  <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///   - [`grant_read_acp(impl Into<String>)`](crate::client::fluent_builders::PutObject::grant_read_acp) / [`set_grant_read_acp(Option<String>)`](crate::client::fluent_builders::PutObject::set_grant_read_acp): <p>Allows grantee to read the object ACL.</p>  <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///   - [`grant_write_acp(impl Into<String>)`](crate::client::fluent_builders::PutObject::grant_write_acp) / [`set_grant_write_acp(Option<String>)`](crate::client::fluent_builders::PutObject::set_grant_write_acp): <p>Allows grantee to write the ACL for the applicable object.</p>  <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///   - [`key(impl Into<String>)`](crate::client::fluent_builders::PutObject::key) / [`set_key(Option<String>)`](crate::client::fluent_builders::PutObject::set_key): <p>Object key for which the PUT action was initiated.</p>
    ///   - [`metadata(HashMap<String, String>)`](crate::client::fluent_builders::PutObject::metadata) / [`set_metadata(Option<HashMap<String, String>>)`](crate::client::fluent_builders::PutObject::set_metadata): <p>A map of metadata to store with the object in S3.</p>
    ///   - [`server_side_encryption(ServerSideEncryption)`](crate::client::fluent_builders::PutObject::server_side_encryption) / [`set_server_side_encryption(Option<ServerSideEncryption>)`](crate::client::fluent_builders::PutObject::set_server_side_encryption): <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    ///   - [`storage_class(StorageClass)`](crate::client::fluent_builders::PutObject::storage_class) / [`set_storage_class(Option<StorageClass>)`](crate::client::fluent_builders::PutObject::set_storage_class): <p>By default, Amazon S3 uses the STANDARD Storage Class to store newly created objects. The STANDARD storage class provides high durability and high availability. Depending on performance needs, you can specify a different Storage Class. Amazon S3 on Outposts only uses the OUTPOSTS Storage Class. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/storage-class-intro.html">Storage Classes</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`website_redirect_location(impl Into<String>)`](crate::client::fluent_builders::PutObject::website_redirect_location) / [`set_website_redirect_location(Option<String>)`](crate::client::fluent_builders::PutObject::set_website_redirect_location): <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata. For information about object metadata, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html">Object Key and Metadata</a>.</p>  <p>In the following example, the request header sets the redirect to an object (anotherPage.html) in the same bucket:</p>  <p> <code>x-amz-website-redirect-location: /anotherPage.html</code> </p>  <p>In the following example, the request header sets the object redirect to another website:</p>  <p> <code>x-amz-website-redirect-location: http://www.example.com/</code> </p>  <p>For more information about website hosting in Amazon S3, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/WebsiteHosting.html">Hosting Websites on Amazon S3</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/how-to-page-redirect.html">How to Configure Website Page Redirects</a>. </p>
    ///   - [`sse_customer_algorithm(impl Into<String>)`](crate::client::fluent_builders::PutObject::sse_customer_algorithm) / [`set_sse_customer_algorithm(Option<String>)`](crate::client::fluent_builders::PutObject::set_sse_customer_algorithm): <p>Specifies the algorithm to use to when encrypting the object (for example, AES256).</p>
    ///   - [`sse_customer_key(impl Into<String>)`](crate::client::fluent_builders::PutObject::sse_customer_key) / [`set_sse_customer_key(Option<String>)`](crate::client::fluent_builders::PutObject::set_sse_customer_key): <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    ///   - [`sse_customer_key_md5(impl Into<String>)`](crate::client::fluent_builders::PutObject::sse_customer_key_md5) / [`set_sse_customer_key_md5(Option<String>)`](crate::client::fluent_builders::PutObject::set_sse_customer_key_md5): <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    ///   - [`ssekms_key_id(impl Into<String>)`](crate::client::fluent_builders::PutObject::ssekms_key_id) / [`set_ssekms_key_id(Option<String>)`](crate::client::fluent_builders::PutObject::set_ssekms_key_id): <p>If <code>x-amz-server-side-encryption</code> is present and has the value of <code>aws:kms</code>, this header specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetrical customer managed key that was used for the object. If you specify <code>x-amz-server-side-encryption:aws:kms</code>, but do not provide<code> x-amz-server-side-encryption-aws-kms-key-id</code>, Amazon S3 uses the Amazon Web Services managed key to protect the data. If the KMS key does not exist in the same account issuing the command, you must use the full ARN and not just the ID. </p>
    ///   - [`ssekms_encryption_context(impl Into<String>)`](crate::client::fluent_builders::PutObject::ssekms_encryption_context) / [`set_ssekms_encryption_context(Option<String>)`](crate::client::fluent_builders::PutObject::set_ssekms_encryption_context): <p>Specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    ///   - [`bucket_key_enabled(bool)`](crate::client::fluent_builders::PutObject::bucket_key_enabled) / [`set_bucket_key_enabled(bool)`](crate::client::fluent_builders::PutObject::set_bucket_key_enabled): <p>Specifies whether Amazon S3 should use an S3 Bucket Key for object encryption with server-side encryption using AWS KMS (SSE-KMS). Setting this header to <code>true</code> causes Amazon S3 to use an S3 Bucket Key for object encryption with SSE-KMS.</p>  <p>Specifying this header with a PUT action doesn’t affect bucket-level settings for S3 Bucket Key.</p>
    ///   - [`request_payer(RequestPayer)`](crate::client::fluent_builders::PutObject::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::client::fluent_builders::PutObject::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`tagging(impl Into<String>)`](crate::client::fluent_builders::PutObject::tagging) / [`set_tagging(Option<String>)`](crate::client::fluent_builders::PutObject::set_tagging): <p>The tag-set for the object. The tag-set must be encoded as URL Query parameters. (For example, "Key1=Value1")</p>
    ///   - [`object_lock_mode(ObjectLockMode)`](crate::client::fluent_builders::PutObject::object_lock_mode) / [`set_object_lock_mode(Option<ObjectLockMode>)`](crate::client::fluent_builders::PutObject::set_object_lock_mode): <p>The Object Lock mode that you want to apply to this object.</p>
    ///   - [`object_lock_retain_until_date(DateTime)`](crate::client::fluent_builders::PutObject::object_lock_retain_until_date) / [`set_object_lock_retain_until_date(Option<DateTime>)`](crate::client::fluent_builders::PutObject::set_object_lock_retain_until_date): <p>The date and time when you want this object's Object Lock to expire. Must be formatted as a timestamp parameter.</p>
    ///   - [`object_lock_legal_hold_status(ObjectLockLegalHoldStatus)`](crate::client::fluent_builders::PutObject::object_lock_legal_hold_status) / [`set_object_lock_legal_hold_status(Option<ObjectLockLegalHoldStatus>)`](crate::client::fluent_builders::PutObject::set_object_lock_legal_hold_status): <p>Specifies whether a legal hold will be applied to this object. For more information about S3 Object Lock, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lock.html">Object Lock</a>.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::client::fluent_builders::PutObject::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::client::fluent_builders::PutObject::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
                            /// - On success, responds with [`PutObjectOutput`](crate::output::PutObjectOutput) with field(s):
    ///   - [`expiration(Option<String>)`](crate::output::PutObjectOutput::expiration): <p>If the expiration is configured for the object (see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycleConfiguration.html">PutBucketLifecycleConfiguration</a>), the response includes this header. It includes the <code>expiry-date</code> and <code>rule-id</code> key-value pairs that provide information about object expiration. The value of the <code>rule-id</code> is URL-encoded.</p>
    ///   - [`e_tag(Option<String>)`](crate::output::PutObjectOutput::e_tag): <p>Entity tag for the uploaded object.</p>
    ///   - [`checksum_crc32(Option<String>)`](crate::output::PutObjectOutput::checksum_crc32): <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_crc32_c(Option<String>)`](crate::output::PutObjectOutput::checksum_crc32_c): <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha1(Option<String>)`](crate::output::PutObjectOutput::checksum_sha1): <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha256(Option<String>)`](crate::output::PutObjectOutput::checksum_sha256): <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`server_side_encryption(Option<ServerSideEncryption>)`](crate::output::PutObjectOutput::server_side_encryption): <p>If you specified server-side encryption either with an Amazon Web Services KMS key or Amazon S3-managed encryption key in your PUT request, the response includes this header. It confirms the encryption algorithm that Amazon S3 used to encrypt the object.</p>
    ///   - [`version_id(Option<String>)`](crate::output::PutObjectOutput::version_id): <p>Version of the object.</p>
    ///   - [`sse_customer_algorithm(Option<String>)`](crate::output::PutObjectOutput::sse_customer_algorithm): <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    ///   - [`sse_customer_key_md5(Option<String>)`](crate::output::PutObjectOutput::sse_customer_key_md5): <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    ///   - [`ssekms_key_id(Option<String>)`](crate::output::PutObjectOutput::ssekms_key_id): <p>If <code>x-amz-server-side-encryption</code> is present and has the value of <code>aws:kms</code>, this header specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object. </p>
    ///   - [`ssekms_encryption_context(Option<String>)`](crate::output::PutObjectOutput::ssekms_encryption_context): <p>If present, specifies the Amazon Web Services KMS Encryption Context to use for object encryption. The value of this header is a base64-encoded UTF-8 string holding JSON with the encryption context key-value pairs.</p>
    ///   - [`bucket_key_enabled(bool)`](crate::output::PutObjectOutput::bucket_key_enabled): <p>Indicates whether the uploaded object uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    ///   - [`request_charged(Option<RequestCharged>)`](crate::output::PutObjectOutput::request_charged): <p>If present, indicates that the requester was successfully charged for the request.</p>
                            /// - On failure, responds with [`SdkError<PutObjectError>`](crate::error::PutObjectError)
    pub fn put_object(&self) -> crate::client::fluent_builders::PutObject {
                                crate::client::fluent_builders::PutObject::new(self.handle.clone())
                            }
}

