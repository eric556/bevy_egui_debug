use proc_macro2::{Ident, TokenStream};
use quote::{quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(EguiDebugDerive)]
pub fn derive_egui_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    
    let function_impls: TokenStream = match input.data {
        Data::Struct(struct_data) => {
            struct_data_impl(struct_data)
        },
        Data::Enum(enum_data) => {
            enum_data_impl(name.clone(), enum_data)
        },
        Data::Union(union_data) => unimplemented!(),
    };

    let expanded = quote! {
        impl bevy_egui_debug::EguiDebug for #name {
            #function_impls
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn struct_data_impl(data: DataStruct) -> TokenStream {

    match data.fields {
        Fields::Named(named_fields) => {
            let field_idents: Vec<(String, Ident)> = named_fields.named.iter().map(|field| {
                let ident = field.ident.as_ref().unwrap().clone();
                (ident.to_string(), ident)
            }).collect();

            named_struct_impl(&field_idents)
        },
        Fields::Unnamed(unamed_fields) => {
            let num_unnamed_fields = unamed_fields.unnamed.len();
            unnamed_struct_impl(num_unnamed_fields)
        },
        Fields::Unit => unimplemented!(),
    }
}

fn named_struct_impl(idents: &Vec<(String, Ident)>) -> TokenStream {
    let names: Vec<&String> = idents.iter().map(|(name, _)| { name }).collect();
    let id: Vec<&Ident> = idents.iter().map(|(_, ident)| { ident }).collect();

    quote! {
        fn debug(&self, ui: &mut bevy_egui::egui::Ui) {
            #(
                ui.horizontal(|ui|{
                    ui.label(#names);
                    self.#id.debug(ui);
                });
            )*
        }
    
        fn debug_mut(&mut self, ui: &mut bevy_egui::egui::Ui) {
            #(
                ui.horizontal(|ui|{
                    ui.label(#names);
                    self.#id.debug_mut(ui);
                });
            )*
        }
    }
}

fn unnamed_struct_impl(num_fields: usize) -> TokenStream {
    let mut debug_statments: Vec<TokenStream> = Vec::new();
    let mut debug_mut_statments: Vec<TokenStream> = Vec::new();

    for i in 0..num_fields {
        let index = syn::Index::from(i);
        debug_statments.push(quote!{
            self.#index.debug(ui);
        });
    }

    for i in 0..num_fields {
        let index = syn::Index::from(i);
        debug_mut_statments.push(quote!{
            self.#index.debug_mut(ui);
        });
    }

    quote! {
        fn debug(&self, ui: &mut bevy_egui::egui::Ui) {
            #(#debug_statments)*
        }
    
        fn debug_mut(&mut self, ui: &mut bevy_egui::egui::Ui) {
            #(#debug_mut_statments)*
        }
    }
}

fn enum_data_impl(name: Ident, data: DataEnum) -> TokenStream{

    let label = format!("{} Type", name.to_string());

    let variant_idents: Vec<Ident> = data.variants.iter().map(|variant| {
        variant.ident.clone()
    }).collect();

    let variant_ident_strings: Vec<String> = variant_idents.iter().map(|variant_ident| {
        variant_ident.to_string()
    }).collect();

    quote! {
        fn debug(&self, ui: &mut bevy_egui::egui::Ui) {
            ui.label(format!("{:?}", self));
        }
    
        fn debug_mut(&mut self, ui: &mut bevy_egui::egui::Ui) {
            bevy_egui::egui::ComboBox::from_label(#label).selected_text(format!("{:?}", self)).show_ui(ui, |ui| {
                    #(
                        ui.selectable_value(self, #name::#variant_idents, #variant_ident_strings);
                    )*
                }
            );
        }
    }
}