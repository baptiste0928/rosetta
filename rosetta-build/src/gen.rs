use std::collections::HashMap;

use heck::CamelCase;
use icu_locid::LanguageIdentifier;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::{
    parser::{TranslationData, TranslationKey},
    RosettaConfig,
};

pub(crate) struct CodeGen<'a> {
    keys: &'a HashMap<String, TranslationKey>,
    languages: Vec<&'a LanguageIdentifier>,
    name: Ident,
}

impl<'a> CodeGen<'a> {
    /// Initialize a new [`CodeGen`]
    pub(crate) fn new(data: &'a TranslationData, config: &'a RosettaConfig) -> Self {
        let name = Ident::new(&config.name, Span::call_site());

        CodeGen {
            keys: &data.keys,
            languages: config.languages(),
            name,
        }
    }

    /// Generate code as a [`TokenStream`]
    pub(crate) fn generate(&self) -> TokenStream {
        // Transform as CamelCase strings
        let languages: Vec<_> = self
            .languages
            .iter()
            .map(|lang| lang.to_string().to_camel_case())
            .collect();

        let name = &self.name;
        let fields = languages
            .iter()
            .map(|lang| Ident::new(lang, Span::call_site()));

        let methods = self.keys.iter().map(|(key, value)| match value {
            TranslationKey::Simple { fallback, others } => {
                self.method_simple(key, fallback, others)
            }
        });

        quote! {
            /// Language type generated by the [rosetta](https://github.com/baptiste0928/rosetta) i18n library.
            #[derive(Debug Clone, Copy, Eq, PartialEq)]
            pub enum #name {
                #(#fields),*
            }

            impl #name {
                #(#methods)*
            }
        }
    }

    /// Generate method for [`TranslationKey::Simple`]
    fn method_simple(
        &self,
        key: &str,
        fallback: &str,
        others: &HashMap<LanguageIdentifier, String>,
    ) -> TokenStream {
        let name = Ident::new(key, Span::call_site());
        let arms = others
            .iter()
            .map(|(language, value)| self.match_arm_simple(language, value));

        quote! {
            pub fn #name(&self) -> &'static str {
                match self {
                    #(#arms),*
                    _ => #fallback
                }
            }
        }
    }

    /// Generate match arm for [`TranslationKey::Simple`]
    fn match_arm_simple(&self, language: &LanguageIdentifier, value: &str) -> TokenStream {
        let name = &self.name;
        let lang = Ident::new(&language.to_string().to_camel_case(), Span::call_site());

        quote! { #name::#lang => #value }
    }
}
