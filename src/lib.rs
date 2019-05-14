#![allow(unused_variables)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, ExprLit, ExprRange, ExprUnary, ItemStruct, ItemTrait, Lit, RangeLimits, Result, Token, UnOp, parse_macro_input,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

trait SimplifyLimit {
    fn simple(self) -> Option<i128>;
    fn extract(expr: Expr) -> i128;
}

// The limits of a range can be any expression;
// currently we only support integer literals as limits
// (otherwise we would need to, uh, depend on Miri or something...)
// so extract the number that the literal represents,
// and produce a compiler error on any more complex expression.
impl SimplifyLimit for Option<Box<Expr>> {
    fn simple(self) -> Option<i128> {
        match self {
            None => None,
            Some(expr) => Some(Self::extract(*expr))
        }
    }

    fn extract(limit_expr: Expr) -> i128 {
        match limit_expr {
            Expr::Lit(ExprLit { lit: Lit::Int(num), .. }) => {
                num.value() as i128
            },
            Expr::Unary(ExprUnary { op: UnOp::Neg(_), expr, .. }) => -Self::extract(*expr),
            _ => panic!("Expressions other than numeric literals are not currently supported")
        }
    }
}

// A range is either open (inclusive of the upper bound)
// or closed (exclusive of the upper bound).
// Currently syn does not have nice support for parsing 128-bit ints,
// so we constrain our support to 64-bit bounds.
// This means that an i128 can hold all supported bounds,
// be they positive or negative.
// A user may also omit one of the bounds,
// in which case the number the bound represents
// cannot be determined until we parse the underlying type,
// so at this stage we use a None to represent an omitted bound.
#[derive(Debug)]
enum Interval { Open, Closed }

#[derive(Debug)]
struct Range {
    lower: Option<i128>,
    upper: Option<i128>,
	interval: Interval,
}

#[derive(Debug)]
struct RangeSeq {
	ranges: Vec<Range>
}

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
				lower: from.simple(),
				upper: to.simple(),
				interval: match limits {
					RangeLimits::HalfOpen(_) => Interval::Open,
					RangeLimits::Closed(_) => Interval::Closed
				}
            });
        }
        Ok(rseq)
    }
}

#[proc_macro_attribute]
pub fn range(attr: TokenStream, item: TokenStream) -> TokenStream {
    //println!("--- attr ---\n{:?}\n", attr);
    //println!("--- item ---\n{:?}\n", item);
    let pattr = parse_macro_input!(attr as RangeSeq);
    //println!("{:#?}", pattr);
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

	let (lower, upper) = match pattr.ranges[0] {
		Range {lower, upper, ..} => (lower.unwrap() as i32, upper.unwrap() as i32)
	};
    // TODO: We're gonna need hygiene SOMEWHERE in here, right?
    let result = quote! {
        struct #name(i32);
        impl #name {
            const LOWER: i32 = #lower;
            const UPPER: i32 = #upper;
        }
    };
    let result: TokenStream = result.into();
    //println!("--- result ---\n{:?}\n", result);
    result
}

#[cfg(test)]
mod tests {
}
