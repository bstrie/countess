#![allow(unused_variables)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprLit, ExprRange, ExprUnary, ItemStruct, Lit, RangeLimits, Result, Token, UnOp, parse_macro_input,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

/// A procedural macro enabling Ada-style range-checked integers.
///
/// ```
/// use countess::valid;
///
/// #[valid(0..10)]
/// struct Foo(i32);
/// ```
#[proc_macro_attribute]
pub fn valid(attr: TokenStream, item: TokenStream) -> TokenStream {
    //dbg!(&attr);
    //dbg!(&item);

    let pattr = parse_macro_input!(attr as RangeSeq);
    //println!("{:#?}", pattr);

    // TODO: parse as custom num newtype
    let input = parse_macro_input!(item as ItemStruct);

    let name = &input.ident;

	let (lower, upper) = match pattr.ranges[0] {
		Range {lower, upper, ..} => (lower.unwrap() as i32, upper.unwrap() as i32)
	};

    // TODO: use declarative macro inside of quote to contain the actual impls
    let result: TokenStream = quote! {
        struct #name(i32);
        impl #name {
            const LOWER: i32 = #lower;
            const UPPER: i32 = #upper;
        }
    }.into();
    //dbg!(&result);

    result
}

#[derive(Debug)]
struct RangeSeq {
	ranges: Vec<Range>
}

// For ease of initial implementation we only support 64-bit numeric types.
// This means that an i128 can hold all supported bounds,
// be they positive or negative.
// A user may also omit one of the bounds,
// in which case the number the bound represents
// cannot be determined until we parse the underlying type,
// so at this stage we use a None to represent an omitted bound.
#[derive(Debug)]
struct Range {
	interval: Interval,
    lower: Option<i128>,
    upper: Option<i128>,
}

// A range is either open (inclusive of the upper bound)
// or closed (exclusive of the upper bound).
#[derive(Debug)]
enum Interval { Open, Closed }

// Implementing syn::Parse is how syn allows one to
// hook into the syn::parse_macro_input! macro.
// This parser should be able to support parsing a list of ranges
// e.g. `1..10, -2..20, ..30, 40..`.
impl Parse for RangeSeq {
    fn parse(input: ParseStream) -> Result<Self> {
        // turn the raw token tree into a series of one or more ExprRanges
        let ranges_ast: Punctuated<ExprRange, Token![,]> = input.parse_terminated(ExprRange::parse)?;

        // turn the ExprRanges into something simpler, our own Range enum
        let mut rseq = RangeSeq { ranges: Vec::new() };
        for ExprRange {limits, from, to, ..} in ranges_ast {
			rseq.ranges.push(Range {
				lower: simplify(from),
				upper: simplify(to),
				interval: match limits {
					RangeLimits::HalfOpen(_) => Interval::Open,
					RangeLimits::Closed(_) => Interval::Closed
				}
            });
        }
        Ok(rseq)
    }
}

// The limits of a range can be any expression;
// currently we only support integer literals as limits.
// Here we extract the number that the literal represents,
// and produce a compiler error on any more complex expression.
fn simplify(limit: Option<Box<Expr>>) -> Option<i128> {
    match limit {
        None => None,
        Some(expr) => Some(extract_val(expr))
    }
}

fn extract_val(limit_expr: Box<Expr>) -> i128 {
    match *limit_expr {
        Expr::Lit(ExprLit { lit: Lit::Int(num), .. }) => {
            num.base10_parse::<i128>().expect("Literal could not be parsed as i128")
        },
        Expr::Unary(ExprUnary { op: UnOp::Neg(_), expr, .. }) => -extract_val(expr),
        _ => panic!("Expressions other than numeric literals are not currently supported")
    }
}
