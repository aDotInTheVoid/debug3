// AUTOGENERATED FILE, DO NOT EDIT
//
// Crate Name: `proc_macro2`
// Crate Version: `1.0.56`
impl crate::Debug for proc_macro2::Delimiter {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Parenthesis => {
                f.debug_tuple("Parenthesis").finish();
            }
            Self::Brace => {
                f.debug_tuple("Brace").finish();
            }
            Self::Bracket => {
                f.debug_tuple("Bracket").finish();
            }
            Self::None => {
                f.debug_tuple("None").finish();
            }
        }
    }
}
// Skipping proc_macro2::Group due to hidden fields
// Skipping proc_macro2::Ident due to hidden fields
// Skipping proc_macro2::LexError due to hidden fields
impl crate::Debug for proc_macro2::LineColumn {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("LineColumn")
            .field("line", &self.line)
            .field("column", &self.column)
            .finish()
    }
}
// Skipping proc_macro2::Literal due to hidden fields
// Skipping proc_macro2::Punct due to hidden fields
impl crate::Debug for proc_macro2::Spacing {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Alone => {
                f.debug_tuple("Alone").finish();
            }
            Self::Joint => {
                f.debug_tuple("Joint").finish();
            }
        }
    }
}
// Skipping proc_macro2::Span due to hidden fields
// Skipping proc_macro2::TokenStream due to hidden fields
// Skiping proc_macro2::TokenTree due to config rule TokenTree
// Skipping proc_macro2::extra::DelimSpan due to hidden fields
// Skipping proc_macro2::token_stream::IntoIter due to hidden fields
