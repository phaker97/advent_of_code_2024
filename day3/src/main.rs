//! Day 3:
//! Scan through Text and look for `mul(X,Y)`, where `X` and `Y` are three-digit numbers.
//!
//! - Task 1:
//!   Then multiply every pair `X`, `Y` and return the sum of all products.
//!   `$ day3 --file-name <file>` to execute.
//! - Task 2:
//!   There are also `do()` and `don't()` string in text.
//!   Everytime a `don't()` appears, discard all pairs until you find a `do()`.
//!   `$ day3 --file-name <file> -c|--conditionals` to execute.
//!
//! **Note**: As three-digit numbers fit into `u16` but not `u8`. I have chosen `u16` as the container.
//! When building the sum, they get upcast to `u64`, which will remain the presentation until the very end.
//! Thus, the result cannot exceed [`u64::MAX`].
//!
use clap::Parser;
use logos::{Lexer, Logos, Source};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(short, long)]
    file_name: PathBuf,

    #[clap(short, long, default_value = "false")]
    conditionals: bool,
}

/// This enum represents the things we are looking for in the text.
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    /// [`u16`] is enough for three-digit numbers in base 10.
    /// This represents all valid multiplication instructions
    #[regex(r"mul\((([1-9][0-9]{0,2})|0),(([1-9][0-9]{0,2})|0)\)", mul_callback)]
    Mul((u16, u16)),

    /// Enables the multiplication instruction
    #[token("do()")]
    Do,

    /// Disables the multiplication instruction
    #[token("don't()")]
    Dont,
}

pub fn mul_callback(lex: &mut Lexer<Token>) -> (u16, u16) {
    let len = lex.slice().len();
    let slice = lex.slice();
    let sep = slice.find(',').unwrap();
    let left = slice[4..sep].parse::<u16>().unwrap();
    let right = slice[sep + 1..len - 1].parse::<u16>().unwrap();
    (left, right)
}

type Acc = u64;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Reading file: {}", args.file_name.display());
    println!(
        "{} conditionals",
        if args.conditionals {
            "Respecting"
        } else {
            "Ignoring"
        }
    );

    let content = std::fs::read_to_string(args.file_name)?;

    let lex = Token::lexer(&content);

    let result: Acc = {
        let tokens = lex.filter_map(|t| t.ok());
        if args.conditionals {
            tokens
                .toggle_on(|t| *t == Token::Do, |t| *t == Token::Dont)
                .filter_map(|t| match t {
                    Token::Mul(tuple) => Some(tuple),
                    _ => None,
                })
                .map(|(l, r)| l as Acc * r as Acc)
                .sum()
        } else {
            tokens
                .filter_map(|t| match t {
                    Token::Mul(tuple) => Some(tuple),
                    _ => None,
                })
                .map(|(l, r)| l as Acc * r as Acc)
                .sum()
        }
    };

    println!("{}", result);

    Ok(())
}

pub struct Toggle<J, I: Iterator<Item = J>, POn: FnMut(&J) -> bool, POff: FnMut(&J) -> bool> {
    iter: I,
    on_function: POn,
    off_function: POff,
    state: bool,
}

impl<J, I: Iterator<Item = J>, POn: FnMut(&J) -> bool, POff: FnMut(&J) -> bool>
    Toggle<J, I, POn, POff>
{
    pub fn new(iter: I, on_function: POn, off_function: POff, initial_state: bool) -> Self {
        Toggle {
            iter,
            on_function,
            off_function,
            state: initial_state,
        }
    }

    pub fn new_on(iter: I, on_function: POn, off_function: POff) -> Self {
        Self::new(iter, on_function, off_function, true)
    }

    pub fn new_off(iter: I, on_function: POn, off_function: POff) -> Self {
        Self::new(iter, on_function, off_function, false)
    }
}

/// An Iterator that lets you discard large chunks of data according to some toggle-rules.
/// When getting the next element, this iterator gets one from the underlying iterator,
/// then checks if the new element changes its state.
/// After that, if the current state is on, the element is returned.
/// If the state is on, the procedure is repeated until an element is found that turns its state on.
///
/// The element that turns a state on will be returned.
/// The element that turns a state off will not be returned.
///
/// While the state is on, the on-function will not be queried.
/// While the state is off, the off-function will not be queried.
impl<J, I: Iterator<Item = J>, POn: FnMut(&J) -> bool, POff: FnMut(&J) -> bool> Iterator
    for Toggle<J, I, POn, POff>
{
    type Item = J;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iter.next() {
            if self.state {
                if (self.off_function)(&v) {
                    self.state = false;
                }
            } else {
                if (self.on_function)(&v) {
                    self.state = true;
                }
            }
            if self.state {
                return Some(v);
            }
        }
        None
    }
}
trait Toggleable<J, I: Iterator<Item = J>, POn: FnMut(&J) -> bool, POff: FnMut(&J) -> bool>:
    Iterator<Item = J> + Sized
{
    fn toggle_on(self, on_function: POn, off_function: POff) -> Toggle<J, I, POn, POff>;
    fn toggle_off(self, on_function: POn, off_function: POff) -> Toggle<J, I, POn, POff>;
    fn toggle(
        self,
        on_function: POn,
        off_function: POff,
        initial_state: bool,
    ) -> Toggle<J, I, POn, POff>;
}

impl<J, I: Iterator<Item = J>, POn: FnMut(&J) -> bool, POff: FnMut(&J) -> bool>
    Toggleable<J, Self, POn, POff> for I
{
    fn toggle_on(self, on_function: POn, off_function: POff) -> Toggle<J, I, POn, POff> {
        Toggle::new_on(self, on_function, off_function)
    }
    fn toggle_off(self, on_function: POn, off_function: POff) -> Toggle<J, I, POn, POff> {
        Toggle::new_off(self, on_function, off_function)
    }
    fn toggle(
        self,
        on_function: POn,
        off_function: POff,
        initial_state: bool,
    ) -> Toggle<J, I, POn, POff> {
        Toggle::new(self, on_function, off_function, initial_state)
    }
}
