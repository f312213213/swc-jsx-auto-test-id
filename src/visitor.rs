use swc_core::ecma::{
    ast::{JSXAttr, JSXAttrName, JSXAttrOrSpread, Str, FnDecl},
    visit::{VisitMut, VisitMutWith},
};
use swc_core::common::SyntaxContext;

pub struct TransformVisitor {
    attribute_name: String,
    jsx_depth: usize,
    current_function: Option<String>,
    has_added_test_id: bool,
}

impl TransformVisitor {
    pub fn new(attribute_name: Option<String>) -> Self {
        Self { 
            attribute_name: attribute_name.unwrap_or_else(|| "data-test-id".to_string()),
            jsx_depth: 0,
            current_function: None,
            has_added_test_id: false,
        }
    }

    pub fn get_attribute_name(&self) -> &str {
        &self.attribute_name
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_fn_decl(&mut self, fn_decl: &mut FnDecl) {
        self.current_function = Some(fn_decl.ident.sym.to_string());
        self.has_added_test_id = false;
        fn_decl.visit_mut_children_with(self);
        self.current_function = None;
    }

    fn visit_mut_jsx_element(&mut self, jsx: &mut swc_core::ecma::ast::JSXElement) {
        self.jsx_depth += 1;
        let current_depth = self.jsx_depth;
        jsx.visit_mut_children_with(self);
        self.jsx_depth -= 1;

        if current_depth == 1 && !self.has_added_test_id {
            let has_test_id = jsx.opening.attrs.iter().any(|attr| {
                if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                    if let JSXAttrName::Ident(name) = &attr.name {
                        return name.sym.as_ref() == self.attribute_name;
                    }
                }
                false
            });

            if !has_test_id {
                if let Some(function_name) = &self.current_function {
                    jsx.opening.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                        span: jsx.span,
                        name: JSXAttrName::Ident(swc_core::ecma::ast::Ident::new(
                            self.attribute_name.clone().into(),
                            jsx.span,
                            SyntaxContext::empty(),
                        ).into()),
                        value: Some(swc_core::ecma::ast::JSXAttrValue::Lit(
                            swc_core::ecma::ast::Lit::Str(Str {
                                span: jsx.span,
                                value: function_name.clone().into(),
                                raw: None,
                            }),
                        )),
                    }));
                    self.has_added_test_id = true;
                }
            }
        }
    }
} 