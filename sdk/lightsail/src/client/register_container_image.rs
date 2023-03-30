// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`RegisterContainerImage`](crate::client::fluent_builders::RegisterContainerImage) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`service_name(impl Into<String>)`](crate::client::fluent_builders::RegisterContainerImage::service_name) / [`set_service_name(Option<String>)`](crate::client::fluent_builders::RegisterContainerImage::set_service_name): <p>The name of the container service for which to register a container image.</p>
    ///   - [`label(impl Into<String>)`](crate::client::fluent_builders::RegisterContainerImage::label) / [`set_label(Option<String>)`](crate::client::fluent_builders::RegisterContainerImage::set_label): <p>The label for the container image when it's registered to the container service.</p>  <p>Use a descriptive label that you can use to track the different versions of your registered container images.</p>  <p>Use the <code>GetContainerImages</code> action to return the container images registered to a Lightsail container service. The label is the <code>   <imagelabel></imagelabel></code> portion of the following image name example:</p>  <ul>   <li> <p> <code>:container-service-1.     <imagelabel>      .1     </imagelabel></code> </p> </li>  </ul>  <p>If the name of your container service is <code>mycontainerservice</code>, and the label that you specify is <code>mystaticwebsite</code>, then the name of the registered container image will be <code>:mycontainerservice.mystaticwebsite.1</code>.</p>  <p>The number at the end of these image name examples represents the version of the registered container image. If you push and register another container image to the same Lightsail container service, with the same label, then the version number for the new registered container image will be <code>2</code>. If you push and register another container image, the version number will be <code>3</code>, and so on.</p>
    ///   - [`digest(impl Into<String>)`](crate::client::fluent_builders::RegisterContainerImage::digest) / [`set_digest(Option<String>)`](crate::client::fluent_builders::RegisterContainerImage::set_digest): <p>The digest of the container image to be registered.</p>
                            /// - On success, responds with [`RegisterContainerImageOutput`](crate::output::RegisterContainerImageOutput) with field(s):
    ///   - [`container_image(Option<ContainerImage>)`](crate::output::RegisterContainerImageOutput::container_image): <p>An object that describes a container image that is registered to a Lightsail container service</p>
                            /// - On failure, responds with [`SdkError<RegisterContainerImageError>`](crate::error::RegisterContainerImageError)
    pub fn register_container_image(&self) -> crate::client::fluent_builders::RegisterContainerImage {
                                crate::client::fluent_builders::RegisterContainerImage::new(self.handle.clone())
                            }
}

