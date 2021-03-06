use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;


pub fn impl_input_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());
    let value = Ident::new(format!("{}_{}", name_str, "value").as_str(), name.span());
    let set_value = Ident::new(
        format!("{}_{}", name_str, "set_value").as_str(),
        name.span(),
    );
    let maximum_size = Ident::new(
        format!("{}_{}", name_str, "maximum_size").as_str(),
        name.span(),
    );
    let set_maximum_size = Ident::new(
        format!("{}_{}", name_str, "set_maximum_size").as_str(),
        name.span(),
    );
    let position = Ident::new(format!("{}_{}", name_str, "position").as_str(), name.span());
    let set_position = Ident::new(
        format!("{}_{}", name_str, "set_position").as_str(),
        name.span(),
    );
    let mark = Ident::new(format!("{}_{}", name_str, "mark").as_str(), name.span());
    let set_mark = Ident::new(format!("{}_{}", name_str, "set_mark").as_str(), name.span());
    let replace = Ident::new(format!("{}_{}", name_str, "replace").as_str(), name.span());
    let insert = Ident::new(format!("{}_{}", name_str, "insert").as_str(), name.span());
    let append = Ident::new(format!("{}_{}", name_str, "append").as_str(), name.span());
    let copy = Ident::new(format!("{}_{}", name_str, "copy").as_str(), name.span());
    let undo = Ident::new(format!("{}_{}", name_str, "undo").as_str(), name.span());
    let copy_cuts = Ident::new(
        format!("{}_{}", name_str, "copy_cuts").as_str(),
        name.span(),
    );
    let text_font = Ident::new(
        format!("{}_{}", name_str, "text_font").as_str(),
        name.span(),
    );
    let set_text_font = Ident::new(
        format!("{}_{}", name_str, "set_text_font").as_str(),
        name.span(),
    );
    let text_color = Ident::new(
        format!("{}_{}", name_str, "text_color").as_str(),
        name.span(),
    );
    let set_text_color = Ident::new(
        format!("{}_{}", name_str, "set_text_color").as_str(),
        name.span(),
    );
    let text_size = Ident::new(
        format!("{}_{}", name_str, "text_size").as_str(),
        name.span(),
    );
    let set_text_size = Ident::new(
        format!("{}_{}", name_str, "set_text_size").as_str(),
        name.span(),
    );
    let readonly = Ident::new(format!("{}_{}", name_str, "readonly").as_str(), name.span());
    let set_readonly = Ident::new(
        format!("{}_{}", name_str, "set_readonly").as_str(),
        name.span(),
    );
    let wrap = Ident::new(format!("{}_{}", name_str, "wrap").as_str(), name.span());
    let set_wrap = Ident::new(format!("{}_{}", name_str, "set_wrap").as_str(), name.span());

    let gen = quote! {
        impl InputExt for #name {
            fn value(&self) -> String {
                unsafe {
                    let value_ptr = #value(self._inner);
                    assert!(!value_ptr.is_null(), "Failed to retrieve input/output value!");
                    CStr::from_ptr(value_ptr as *mut raw::c_char).to_string_lossy().to_string()
                }
            }

            fn set_value(&self, val: &str) {
                let temp = CString::new(val).unwrap();
                unsafe {
                    #set_value(self._inner, temp.into_raw() as *const raw::c_char);
                }
            }

            fn maximum_size(&self) -> u32 {
                unsafe {
                    #maximum_size(self._inner) as u32
                }
            }

            fn set_maximum_size(&mut self, val: u32) {
                unsafe {
                    debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    #set_maximum_size(self._inner, val as i32)
                }
            }

            fn position(&self) -> i32 {
                unsafe {
                    #position(self._inner)
                }
            }

            fn set_position(&mut self, val: i32) {
                unsafe {
                    #set_position(self._inner, val);
                }
            }

            fn mark(&self) -> i32 {
                unsafe {
                    #mark(self._inner)
                }
            }

            fn set_mark(&mut self, val: i32) {
                unsafe {
                    #set_mark(self._inner, val);
                }
            }

            fn replace(&mut self, beg: u32, end: u32, val: &str) {
                let val = CString::new(val).unwrap();
                unsafe {
                    debug_assert!(beg <= std::i32::MAX as u32 && end <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    #replace(self._inner, beg as i32, end as i32, val.into_raw() as *const raw::c_char, 0);
                }
            }

            fn insert(&mut self, txt: &str) {
                let txt = CString::new(txt).unwrap();
                unsafe {
                    #insert(self._inner, txt.into_raw() as *const raw::c_char, 0);
                }
            }

            fn append(&mut self, txt: &str) {
                let txt = CString::new(txt).unwrap();
                unsafe {
                    #append(self._inner,  txt.into_raw() as *const raw::c_char, 0, 0);
                }
            }

            fn copy(&mut self) {
                unsafe {
                    #copy(self._inner, 1);
                }
            }

            fn undo(&mut self) {
                unsafe {
                    #undo(self._inner);
                }
            }

            fn cut(&mut self) {
                unsafe {
                    #copy_cuts(self._inner);
                }
            }

            fn text_font(&self) -> Font {
                unsafe {
                    mem::transmute(#text_font(self._inner))
                }
            }

            fn set_text_font(&mut self, font: Font) {
                unsafe {
                    #set_text_font(self._inner, font as i32)
                }
            }

            fn text_color(&self) -> Color {
                unsafe {
                    mem::transmute(#text_color(self._inner))
                }
            }

            fn set_text_color(&mut self, color: Color) {
                unsafe {
                    #set_text_color(self._inner, color as u32)
                }
            }

            fn text_size(&self) -> u32 {
                unsafe {
                    #text_size(self._inner) as u32
                }
            }

            fn set_text_size(&mut self, sz: u32) {
                unsafe {
                    debug_assert!(sz <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                    #set_text_size(self._inner, sz as i32)
                }
            }

            fn readonly(&self) -> bool {
                unsafe {
                    match #readonly(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_readonly(&mut self, val: bool) {
                unsafe {
                    #set_readonly(self._inner, val as i32)
                }
            }

            fn wrap(&self) -> bool {
                unsafe {
                    match #wrap(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_wrap(&mut self, val: bool) {
                unsafe {
                    #set_wrap(self._inner, val as i32)
                }
            }
        }
    };
    gen.into()
}