use swc_core::ecma::{
    ast::{JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXElementName, Str},
    visit::{VisitMut, VisitMutWith},
};
use swc_core::common::SyntaxContext;

pub struct TransformVisitor {
    attribute_name: String,
    current_function_name: Option<String>,
    nesting_level: usize,
    in_fragment: bool,
    found_first_in_fragment: bool,
}

impl TransformVisitor {
    pub fn new(attribute_name: Option<String>) -> Self {
        Self { 
            attribute_name: attribute_name.unwrap_or_else(|| "data-test-id".to_string()),
            current_function_name: None,
            nesting_level: 0,
            in_fragment: false,
            found_first_in_fragment: false,
        }
    }

    pub fn get_attribute_name(&self) -> &str {
        &self.attribute_name
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_fn_decl(&mut self, fn_decl: &mut swc_core::ecma::ast::FnDecl) {
        let prev_function_name = self.current_function_name.clone();
        if fn_decl.ident.sym.chars().next().map_or(false, |c| c.is_uppercase()) {
            self.current_function_name = Some(fn_decl.ident.sym.to_string());
            self.nesting_level = 0;  // Reset nesting level for new function
        }
        fn_decl.visit_mut_children_with(self);
        self.current_function_name = prev_function_name;
    }

    fn visit_mut_var_decl(&mut self, var_decl: &mut swc_core::ecma::ast::VarDecl) {
        let prev_function_name = self.current_function_name.clone();
        if let Some(decl) = var_decl.decls.first() {
            if let Some(init) = &decl.init {
                if let swc_core::ecma::ast::Expr::Arrow(_) = &**init {
                    if let swc_core::ecma::ast::Pat::Ident(ident) = &decl.name {
                        if ident.id.sym.chars().next().map_or(false, |c| c.is_uppercase()) {
                            self.current_function_name = Some(ident.id.sym.to_string());
                            self.nesting_level = 0;  // Reset nesting level for new function
                        }
                    }
                }
            }
        }
        var_decl.visit_mut_children_with(self);
        self.current_function_name = prev_function_name;
    }

    fn visit_mut_jsx_element(&mut self, jsx: &mut swc_core::ecma::ast::JSXElement) {
        self.nesting_level += 1;
        
        if let JSXElementName::Ident(ident) = &jsx.opening.name {
            let has_test_id = jsx.opening.attrs.iter().any(|attr| {
                if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                    if let JSXAttrName::Ident(name) = &attr.name {
                        return name.sym.as_ref() == self.attribute_name;
                    }
                }
                false
            });

            let is_component = ident.sym.chars().next().map_or(false, |c| c.is_uppercase());
            let should_add_test_id = if self.in_fragment {
                !self.found_first_in_fragment && is_component && self.nesting_level == 2
            } else {
                self.nesting_level == 1
            };

            if !has_test_id && should_add_test_id {
                if let Some(function_name) = &self.current_function_name {
                    jsx.opening.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                        span: jsx.opening.span,
                        name: JSXAttrName::Ident(swc_core::ecma::ast::Ident::new(
                            self.attribute_name.clone().into(),
                            jsx.opening.span,
                            SyntaxContext::empty(),
                        ).into()),
                        value: Some(swc_core::ecma::ast::JSXAttrValue::Lit(
                            swc_core::ecma::ast::Lit::Str(Str {
                                span: jsx.opening.span,
                                value: function_name.clone().into(),
                                raw: None,
                            }),
                        )),
                    }));

                    if self.in_fragment && is_component {
                        self.found_first_in_fragment = true;
                    }
                }
            }
        }

        jsx.visit_mut_children_with(self);
        self.nesting_level -= 1;
    }

    fn visit_mut_jsx_fragment(&mut self, fragment: &mut swc_core::ecma::ast::JSXFragment) {
        let prev_in_fragment = self.in_fragment;
        let prev_found_first = self.found_first_in_fragment;
        let prev_nesting = self.nesting_level;
        
        self.in_fragment = true;
        self.found_first_in_fragment = false;
        self.nesting_level = 1;  // Reset nesting level for fragment contents
        
        fragment.children.visit_mut_with(self);
        
        self.in_fragment = prev_in_fragment;
        self.found_first_in_fragment = prev_found_first;
        self.nesting_level = prev_nesting;
    }
} 