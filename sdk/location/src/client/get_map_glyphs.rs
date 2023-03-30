// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client  {
    /// Constructs a fluent builder for the [`GetMapGlyphs`](crate::client::fluent_builders::GetMapGlyphs) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`map_name(impl Into<String>)`](crate::client::fluent_builders::GetMapGlyphs::map_name) / [`set_map_name(Option<String>)`](crate::client::fluent_builders::GetMapGlyphs::set_map_name): <p>The map resource associated with the glyph ﬁle.</p>
    ///   - [`font_stack(impl Into<String>)`](crate::client::fluent_builders::GetMapGlyphs::font_stack) / [`set_font_stack(Option<String>)`](crate::client::fluent_builders::GetMapGlyphs::set_font_stack): <p>A comma-separated list of fonts to load glyphs from in order of preference. For example, <code>Noto Sans Regular, Arial Unicode</code>.</p>  <p>Valid fonts stacks for <a href="https://docs.aws.amazon.com/location/latest/developerguide/esri.html">Esri</a> styles: </p>  <ul>   <li> <p>VectorEsriDarkGrayCanvas – <code>Ubuntu Medium Italic</code> | <code>Ubuntu Medium</code> | <code>Ubuntu Italic</code> | <code>Ubuntu Regular</code> | <code>Ubuntu Bold</code> </p> </li>   <li> <p>VectorEsriLightGrayCanvas – <code>Ubuntu Italic</code> | <code>Ubuntu Regular</code> | <code>Ubuntu Light</code> | <code>Ubuntu Bold</code> </p> </li>   <li> <p>VectorEsriTopographic – <code>Noto Sans Italic</code> | <code>Noto Sans Regular</code> | <code>Noto Sans Bold</code> | <code>Noto Serif Regular</code> | <code>Roboto Condensed Light Italic</code> </p> </li>   <li> <p>VectorEsriStreets – <code>Arial Regular</code> | <code>Arial Italic</code> | <code>Arial Bold</code> </p> </li>   <li> <p>VectorEsriNavigation – <code>Arial Regular</code> | <code>Arial Italic</code> | <code>Arial Bold</code> </p> </li>  </ul>  <p>Valid font stacks for <a href="https://docs.aws.amazon.com/location/latest/developerguide/HERE.html">HERE Technologies</a> styles:</p>  <ul>   <li> <p>VectorHereContrast – <code>Fira GO Regular</code> | <code>Fira GO Bold</code> </p> </li>   <li> <p>VectorHereExplore, VectorHereExploreTruck, HybridHereExploreSatellite – <code>Fira GO Italic</code> | <code>Fira GO Map</code> | <code>Fira GO Map Bold</code> | <code>Noto Sans CJK JP Bold</code> | <code>Noto Sans CJK JP Light</code> | <code>Noto Sans CJK JP Regular</code> </p> </li>  </ul>  <p>Valid font stacks for <a href="https://docs.aws.amazon.com/location/latest/developerguide/grab.html">GrabMaps</a> styles:</p>  <ul>   <li> <p>VectorGrabStandardLight, VectorGrabStandardDark – <code>Noto Sans Regular</code> | <code>Noto Sans Medium</code> | <code>Noto Sans Bold</code> </p> </li>  </ul>  <p>Valid font stacks for <a href="https://docs.aws.amazon.com/location/latest/developerguide/open-data.html">Open Data (Preview)</a> styles:</p>  <ul>   <li> <p>VectorOpenDataStandardLight – <code>Amazon Ember Regular,Noto Sans Regular</code> | <code>Amazon Ember Bold,Noto Sans Bold</code> | <code>Amazon Ember Medium,Noto Sans Medium</code> | <code>Amazon Ember Regular Italic,Noto Sans Italic</code> | <code>Amazon Ember Condensed RC Regular,Noto Sans Regular</code> | <code>Amazon Ember Condensed RC Bold,Noto Sans Bold</code> </p> </li>  </ul> <note>   <p>The fonts used by <code>VectorOpenDataStandardLight</code> are combined fonts that use <code>Amazon Ember</code> for most glyphs but <code>Noto Sans</code> for glyphs unsupported by <code>Amazon Ember</code>.</p>  </note>
    ///   - [`font_unicode_range(impl Into<String>)`](crate::client::fluent_builders::GetMapGlyphs::font_unicode_range) / [`set_font_unicode_range(Option<String>)`](crate::client::fluent_builders::GetMapGlyphs::set_font_unicode_range): <p>A Unicode range of characters to download glyphs for. Each response will contain 256 characters. For example, 0–255 includes all characters from range <code>U+0000</code> to <code>00FF</code>. Must be aligned to multiples of 256.</p>
                            /// - On success, responds with [`GetMapGlyphsOutput`](crate::output::GetMapGlyphsOutput) with field(s):
    ///   - [`blob(Option<Blob>)`](crate::output::GetMapGlyphsOutput::blob): <p>The blob's content type.</p>
    ///   - [`content_type(Option<String>)`](crate::output::GetMapGlyphsOutput::content_type): <p>The map glyph content type. For example, <code>application/octet-stream</code>.</p>
                            /// - On failure, responds with [`SdkError<GetMapGlyphsError>`](crate::error::GetMapGlyphsError)
    pub fn get_map_glyphs(&self) -> crate::client::fluent_builders::GetMapGlyphs {
                                crate::client::fluent_builders::GetMapGlyphs::new(self.handle.clone())
                            }
}

