use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(VecDerive)]
pub fn vec_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(n) => n.named,
            syn::Fields::Unnamed(n) => n.unnamed,
            syn::Fields::Unit => panic!("You can't implement Vec on that"),
        },
        _ => panic!("You can't implement Vec on that"),
    };

    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let output = quote!(
        impl #ident {
            pub fn new(#(#field_names: f32), *) -> Self {
                Self {
                    #(#field_names), *
                }
            }

            pub fn length(&self) -> f32 {
                (#(self.#field_names * self.#field_names) + * ).sqrt()
            }

            pub fn normalize(&self) -> Self {
                let len = self.length();
                *self / len
            }

            pub fn get_angle_between(self, rhs: Self) -> f32 {
                let lhs_length = self.length();
                let rhs_length = rhs.length();
                (dbg!(self * rhs) / dbg!(lhs_length * rhs_length)).acos()
            }
        }

        impl std::ops::Add for #ident {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self {
                    #(#field_names: self.#field_names + rhs.#field_names), *
                }
            }
        }

        impl std::ops::AddAssign for #ident {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl std::ops::Sub for #ident {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self {
                    #(#field_names: self.#field_names - rhs.#field_names), *
                }
            }
        }

        impl std::ops::SubAssign for #ident {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl std::ops::Mul for #ident {
            type Output = f32;

            fn mul(self, rhs: Self) -> Self::Output {
                dbg!(#(self.#field_names * rhs.#field_names) + *)
            }
        }

        impl std::ops::Mul<f32> for #ident {
            type Output = #ident;

            fn mul(self, rhs: f32) -> Self::Output {
                Self {
                    #(#field_names: self.#field_names * rhs), *
                }
            }
        }

        impl std::ops::MulAssign<f32> for #ident {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }

        impl std::ops::Div<f32> for #ident {
            type Output = #ident;

            fn div(self, rhs: f32) -> Self::Output {
                Self {
                    #(#field_names: self.#field_names / rhs), *
                }
            }
        }

        impl std::ops::DivAssign<f32> for #ident {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }

        impl From<f32> for #ident {
            fn from(value: f32) -> Self {
                Self { #(#field_names: value), * }
            }
        }
    );

    output.into()
}
