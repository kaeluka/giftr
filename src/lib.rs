pub mod refs;

//#![crate_type="dylib"]
//#![feature(plugin_registrar, rustc_private)]
//#![feature(box_syntax)]
//
//
//#[macro_use]
//extern crate rustc;
//extern crate rustc_plugin;
//extern crate syntax;
//
//mod refs;
//
//use rustc_plugin::Registry;
//use syntax::codemap::Span;
//use syntax::parse::token;
//use syntax::ast;
//use syntax::ast::TokenTree;
//use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
//use syntax::ext::build::AstBuilder;  // trait for expr_usize

//use rustc::lint::{EarlyContext, LintContext, LintPass, EarlyLintPass, EarlyLintPassObject,
//                  LateContext, LateLintPass, LateLintPassObject,
//                  LintArray};
//use rustc::hir::intravisit::FnKind;
//use rustc::hir::{FnDecl, Block, Decl};
//use rustc::ty::TyCtxt;
//
//declare_lint!(TEST_LINT, Warn, "Warn about items named 'foo'");
//
//struct FooPass;
//
//impl LintPass for FooPass {
//    fn get_lints(&self) -> LintArray {
//        lint_array!(TEST_LINT)
//    }
//}
//
//impl EarlyLintPass for FooPass {
//    fn check_item(&mut self, cx: &EarlyContext, it: &ast::Item) {
////        println!("it = {:?}\n------------------------", it);
//        if it.ident.name.as_str() == "foo" {
//            cx.span_lint(TEST_LINT, it.span, "item is named 'foo'");
//        }
//    }
//}
//
//
//declare_lint!(FUN_TYPE_LINT, Warn, "Analyse function signatures");
//
//struct FunctionTypePass;
//
//impl LintPass for FunctionTypePass {
//    fn get_lints(&self) -> LintArray {
//        lint_array!(FUN_TYPE_LINT)
//    }
//}
//
//impl LateLintPass for FunctionTypePass {
//
//    fn check_fn(&mut self, ctx: &LateContext,
//                _: FnKind, decl: &FnDecl, code: &Block, span: Span, _: ast::NodeId) {
//        println!("vvvvvvvvvvvvvvvvvvvv {:?}", span);
//
//        for inp in &decl.inputs {
//            println!("IN {:?} : {:?}", inp.pat, inp.ty);
//        }
//
//
////        println!("ctx: {:?}", ctx.tcx.);
//
//        for stat in &code.stmts {
//            println!("--> {:?}", stat.node);
//        }
//
//        println!("--> RET {:?}", code.expr);
//        println!("^^^^^^^^^^^^^^^^^^^^");
//    }
//}
//
//#[plugin_registrar]
//pub fn plugin_registrar(_reg: &mut Registry) {
////    reg.register_macro("rn", expand_rn);
////    reg.register_early_lint_pass(box FooPass as EarlyLintPassObject);
//    reg.register_late_lint_pass(box FunctionTypePass as LateLintPassObject)
//}
