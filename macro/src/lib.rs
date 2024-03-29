//! Wrapper crate for [`termcolor_output_impl`] procedural macro.
//!
//! The reason for this code to be split into two crates is simple: we want to make
//! this functionality available on stable. In fact, this dual-crate system is simply
//! the manual implementation of the code generated by [`proc_macro_hack`].
//!
//! ## What is it
//!
//! The [`termcolor`] crate is a cross-platform implementation for the different console
//! APIs, abstracting away both Linux terminals and Windows consoles. It has, however,
//! a but cumbersome API itself (only a bit though), since for formatting-heavy parts
//! we have to litter our code with explicit styling commands. This crate allows to
//! abstract these things away, providing the interface similar to the standard [`write!`]
//! macro.
//!  
//! [`termcolor_output_impl`]: http://crates.io/crates/termcolor_output_impl
//! [`proc_macro_hack`]: http://github.com/dtolnay/proc-macro-hack
//! [`termcolor`]: http://docs.rs/termcolor
//! [`write!`]: https://doc.rust-lang.org/stable/std/macro.write.html

/// The macro writing colored text.
///
/// Like the standard [`write!`] macro, it takes the writer, format string and the sequence of
/// arguments. The arguments may be either formattable with the corresponding formatter (`Display`
/// for `{}`, `Debug` for `{:?}`, etc.), or the _control sequences_, which are written in
/// macro-like style:
/// - `reset!()` yields call to [`ColorSpec::clear`][termcolor::ColorSpec::clear];
/// - `fg!(color)`, `bg!(color)`, `bold!(bool)`, `underline!(bool)` and `intense!(bool)` are
/// translated into corresponding `ColorSpec::set_*` calls with the provided arguments.
///
/// Internally, this expands to the following:
/// - imports of all necessary traits;
/// - call to the `guard` method on the [`WriteColorGuard`] trait (as a sanity check);
/// - an immediately called closure, containing:
///   - creation of `ColorSpec`;
///   - calls to `write!` for every formattable input;
///   - updates for `ColorSpec` for every control sequence.
/// Every error generated inside the closure is returned early and yielded by the macro as an
/// [`std::io::Result<()>`].
///
/// When the arguments list is malformed, macro generates a compile error trying to point on the
/// exact origin.
///
/// ## Examples
///
/// Simple formatting is provided in exactly the same way as for standard writes:
/// ```
/// # use termcolor_output::colored;
/// # fn write(writer: &mut impl termcolor::WriteColor) {
/// colored!(writer, "This text is {} styled", "not").unwrap();
/// # }
/// ```
///
/// Styled formatting is provided by using any formatter argument in format string, wherever you
/// need to apply the style:
/// ```
/// # use termcolor_output::colored;
/// # fn write(writer: &mut impl termcolor::WriteColor) {
/// # use termcolor::Color;
/// colored!(writer, "This text is not styled\n{}And this is colored", fg!(Some(Color::Blue))).unwrap();
/// # }
/// ```
///
/// You can chain several styling commands by specifying several formatter arguments without text
/// between them:
/// ```
/// # use termcolor_output::colored;
/// # fn write(writer: &mut impl termcolor::WriteColor) {
/// # use termcolor::Color;
/// colored!(
///     writer,
///     "{}{}{}This text is bold blue on yellow background
///      {}{}{}And this has default colors, but is bold and underlined",
///     fg!(Some(Color::Blue)), bg!(Some(Color::Yellow)), bold!(true),
///     fg!(None), bg!(None), underline!(true),
/// ).unwrap();
/// # }
/// ```
/// Note that the `bold` being set in the first block of control sequences is preserved after the
/// second one.
///
/// And, of course, you can mix ordinary formatting outputs with the control sequences:
///
/// ```
/// # use termcolor_output::colored;
/// # fn write(writer: &mut impl termcolor::WriteColor) {
/// # use termcolor::Color;
/// colored!(writer, "{}{:?}{} unwraps to {}", bold!(true), Some(0), bold!(false), 0).unwrap();
/// # }
/// ```
///
/// [`write!`]: https://doc.rust-lang.org/std/macro.write.html
/// [`std::io::Result<()>`]: https://doc.rust-lang.org/std/io/type.Result.html
pub use termcolor_output_impl::colored_generate as colored;

/// A convenience function, serving the role of `writeln!` macro.
///
/// This function accepts a closure containing all necessary [`colored`]! calls.
/// It will clear the writer style, run the closure, clear the writer style again
/// and write a newline.
#[allow(unused_imports)]
pub fn colored_ln<W: termcolor::WriteColor, F: FnOnce(&mut W) -> std::io::Result<()>>(
    buf: &mut W,
    func: F,
) -> std::io::Result<()> {
    colored!(buf, "{}", reset!())?;
    func(buf)?;
    colored!(buf, "{}\n", reset!())?;
    Ok(())
}
