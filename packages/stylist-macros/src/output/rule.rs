use super::{
    fragment_coalesce, ContextRecorder, IntoCowVecTokens, OutputFragment, OutputRuleContent, Reify,
};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug)]
pub struct OutputRule {
    pub condition: Vec<OutputFragment>,
    pub content: Vec<OutputRuleContent>,
}

impl Reify for OutputRule {
    fn into_token_stream(self, ctx: &mut ContextRecorder) -> TokenStream {
        let condition = self
            .condition
            .into_iter()
            .coalesce(fragment_coalesce)
            .into_cow_vec_tokens(ctx);
        let content = self.content.into_cow_vec_tokens(ctx);

        quote! {
            ::stylist::ast::Rule {
                condition: {
                    #condition
                },
                content: #content,
            }
        }
    }
}