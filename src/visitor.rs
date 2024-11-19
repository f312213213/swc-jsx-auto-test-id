use swc_core::ecma::ast::{
    FnDecl, Ident, JSXAttr, JSXAttrName, JSXAttrOrSpread, JSXAttrValue,
    JSXElement, JSXElementName, JSXFragment, Lit, Module, Pat, Program, Str, VarDecl, ArrowExpr
};
use swc_core::ecma::visit::{VisitMut, VisitMutWith, Fold};
use swc_core::common::SyntaxContext;
use swc_core::ecma::ast::Pass;

pub struct TransformVisitor {
    attribute_name: String,
    function_name_stack: Vec<String>,
    nesting_level: usize,
    in_fragment: bool,
    found_first_in_fragment: bool,
}

impl Pass for TransformVisitor {
    fn process(&mut self, program: &mut Program) {
        program.visit_mut_with(self);
    }
}

impl TransformVisitor {
    pub fn new(attribute_name: Option<String>) -> Self {
        Self {
            attribute_name: attribute_name.unwrap_or_else(|| "data-test-id".to_string()),
            function_name_stack: Vec::new(),
            nesting_level: 0,
            in_fragment: false,
            found_first_in_fragment: false,
        }
    }
    #[cfg(test)]
    pub fn get_attribute_name(&self) -> &str {
        &self.attribute_name
    }

    fn should_add_test_id(&self, jsx: &JSXElement) -> bool {
        if let JSXElementName::Ident(_) = &jsx.opening.name {
            let has_test_id = jsx.opening.attrs.iter().any(|attr| {
                if let JSXAttrOrSpread::JSXAttr(attr) = attr {
                    if let JSXAttrName::Ident(name) = &attr.name {
                        return name.sym.as_ref() == self.attribute_name;
                    }
                }
                false
            });

            !has_test_id && 
            !self.function_name_stack.is_empty() && 
            self.nesting_level == 0
        } else {
            false
        }
    }

    fn add_test_id(&self, jsx: &mut JSXElement) {
        if let Some(function_name) = self.function_name_stack.last() {
            jsx.opening.attrs.push(JSXAttrOrSpread::JSXAttr(JSXAttr {
                span: jsx.opening.span,
                name: JSXAttrName::Ident(Ident::new(
                    self.attribute_name.clone().into(),
                    jsx.opening.span,
                    SyntaxContext::empty(),
                ).into()),
                value: Some(JSXAttrValue::Lit(Lit::Str(Str {
                    span: jsx.opening.span,
                    value: function_name.clone().into(),
                    raw: None,
                }))),
            }));
        }
    }
}

impl Fold for TransformVisitor {
    fn fold_module(&mut self, mut module: Module) -> Module {
        module.visit_mut_with(self);
        module
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_fn_decl(&mut self, fn_decl: &mut FnDecl) {
        let should_process = fn_decl.ident.sym.chars().next().map_or(false, |c| c.is_uppercase());
        if should_process {
            self.function_name_stack.push(fn_decl.ident.sym.to_string());
            self.nesting_level = 0;
            fn_decl.function.visit_mut_with(self);
            self.function_name_stack.pop();
        } else {
            fn_decl.function.visit_mut_with(self);
        }
    }

    fn visit_mut_var_decl(&mut self, var_decl: &mut VarDecl) {
        for decl in &mut var_decl.decls {
            if let Some(init) = &mut decl.init {
                if let Pat::Ident(ident) = &decl.name {
                    if ident.sym.chars().next().map_or(false, |c| c.is_uppercase()) {
                        self.function_name_stack.push(ident.sym.to_string());
                        self.nesting_level = 0;
                        init.visit_mut_with(self);
                        self.function_name_stack.pop();
                    } else {
                        init.visit_mut_with(self);
                    }
                } else {
                    init.visit_mut_with(self);
                }
            }
        }
    }

    fn visit_mut_jsx_element(&mut self, jsx: &mut JSXElement) {
        if self.should_add_test_id(jsx) {
            self.add_test_id(jsx);
        }

        self.nesting_level += 1;
        
        jsx.opening.visit_mut_with(self);
        jsx.children.visit_mut_with(self);
        if let Some(closing) = &mut jsx.closing {
            closing.visit_mut_with(self);
        }

        self.nesting_level -= 1;
    }

    fn visit_mut_jsx_fragment(&mut self, fragment: &mut JSXFragment) {
        let prev_in_fragment = self.in_fragment;
        let prev_found_first = self.found_first_in_fragment;
        
        self.in_fragment = true;
        self.found_first_in_fragment = false;
        self.nesting_level = 0;
        
        for child in &mut fragment.children {
            child.visit_mut_with(self);
        }
        
        self.in_fragment = prev_in_fragment;
        self.found_first_in_fragment = prev_found_first;
    }

    fn visit_mut_arrow_expr(&mut self, arrow: &mut ArrowExpr) {
        let is_render_method = arrow
            .params
            .iter()
            .any(|param| {
                if let Pat::Ident(ident) = param {
                    ident.sym.to_string().starts_with("render")
                } else {
                    false
                }
            });

        if is_render_method {
            self.function_name_stack.push("RenderComponent".to_string());
            self.nesting_level = 0;
            arrow.body.visit_mut_with(self);
            self.function_name_stack.pop();
        } else {
            arrow.body.visit_mut_with(self);
        }
    }
} 