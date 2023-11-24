#![doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
#![doc(rust_logo)]
#![allow(internal_features)]
#![feature(rustdoc_internals)]
#![feature(proc_macro_diagnostic)]
#![feature(proc_macro_span)]
#![deny(rustc::untranslatable_diagnostic)]
#![deny(rustc::diagnostic_outside_of_impl)]
#![allow(rustc::default_hash_types)]

use proc_macro::TokenStream;

mod fluent;

/// Implements the `fluent_messages` macro, which performs compile-time validation of the
/// compiler's Fluent resources (i.e. that the resources parse and don't multiply define the same
/// messages) and generates constants that make using those messages in diagnostics more ergonomic.
///
/// For example, given the following invocation of the macro..
///
/// ```ignore (rust)
/// fluent_messages! { "./typeck.ftl" }
/// ```
/// ..where `typeck.ftl` has the following contents..
///
/// ```fluent
/// typeck_field_multiply_specified_in_initializer =
///     field `{$ident}` specified more than once
///     .label = used more than once
///     .label_previous_use = first use of `{$ident}`
/// ```
/// ...then the macro parse the Fluent resource, emitting a diagnostic if it fails to do so, and
/// will generate the following code:
///
/// ```ignore (rust)
/// pub static DEFAULT_LOCALE_RESOURCE: &'static [&'static str] = include_str!("./typeck.ftl");
///
/// mod fluent_generated {
///     mod typeck {
///         pub const field_multiply_specified_in_initializer: DiagnosticMessage =
///             DiagnosticMessage::fluent("typeck_field_multiply_specified_in_initializer");
///         pub const field_multiply_specified_in_initializer_label_previous_use: DiagnosticMessage =
///             DiagnosticMessage::fluent_attr(
///                 "typeck_field_multiply_specified_in_initializer",
///                 "previous_use_label"
///             );
///     }
/// }
/// ```
/// When emitting a diagnostic, the generated constants can be used as follows:
///
/// ```ignore (rust)
/// let mut err = sess.struct_span_err(
///     span,
///     fluent::typeck::field_multiply_specified_in_initializer
/// );
/// err.span_default_label(span);
/// err.span_label(
///     previous_use_span,
///     fluent::typeck::field_multiply_specified_in_initializer_label_previous_use
/// );
/// err.emit();
/// ```
#[proc_macro]
pub fn fluent_messages(input: TokenStream) -> TokenStream {
    fluent::fluent_messages(input)
}
