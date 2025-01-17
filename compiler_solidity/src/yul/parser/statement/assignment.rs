//!
//! The assignment expression statement.
//!

use crate::yul::lexer::lexeme::symbol::Symbol;
use crate::yul::lexer::lexeme::Lexeme;
use crate::yul::lexer::Lexer;
use crate::yul::parser::identifier::Identifier;
use crate::yul::parser::statement::expression::Expression;

///
/// The assignment expression statement.
///
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    /// The variable bindings.
    pub bindings: Vec<String>,
    /// The initializing expression.
    pub initializer: Expression,
}

impl Assignment {
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Lexeme>) -> anyhow::Result<Self> {
        let lexeme = crate::yul::parser::take_or_next(initial, lexer)?;

        let identifier = match lexeme {
            Lexeme::Identifier(identifier) => identifier,
            lexeme => {
                anyhow::bail!("Expected one of {:?}, found `{}`", ["{identifier}"], lexeme);
            }
        };

        match lexer.peek()? {
            Lexeme::Symbol(Symbol::Assignment) => {
                lexer.next()?;

                Ok(Self {
                    bindings: vec![identifier],
                    initializer: Expression::parse(lexer, None)?,
                })
            }
            Lexeme::Symbol(Symbol::Comma) => {
                let (identifiers, next) =
                    Identifier::parse_list(lexer, Some(Lexeme::Identifier(identifier)))?;

                match crate::yul::parser::take_or_next(next, lexer)? {
                    Lexeme::Symbol(Symbol::Assignment) => {}
                    lexeme => {
                        anyhow::bail!("Expected one of {:?}, found `{}`", [":="], lexeme);
                    }
                }

                Ok(Self {
                    bindings: identifiers,
                    initializer: Expression::parse(lexer, None)?,
                })
            }
            lexeme => anyhow::bail!("Expected one of {:?}, found `{}`", [":=", ","], lexeme),
        }
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for Assignment
where
    D: compiler_llvm_context::Dependency,
{
    fn into_llvm(mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        let value = match self.initializer.into_llvm(context)? {
            Some(value) => value,
            None => return Ok(()),
        };

        if self.bindings.len() == 1 {
            let name = self.bindings.remove(0);
            context.build_store(context.function().stack[name.as_str()], value.to_llvm());
            return Ok(());
        }

        let llvm_type = value.to_llvm().into_struct_value().get_type();
        let pointer = context.build_alloca(llvm_type, "assignment_pointer");
        context.build_store(pointer, value.to_llvm());

        for (index, binding) in self.bindings.into_iter().enumerate() {
            let pointer = unsafe {
                context.builder().build_gep(
                    pointer,
                    &[
                        context.field_const(0),
                        context
                            .integer_type(compiler_common::BITLENGTH_X32)
                            .const_int(index as u64, false),
                    ],
                    format!("assignment_binding_{}_gep_pointer", index).as_str(),
                )
            };

            let value = context.build_load(
                pointer,
                format!("assignment_binding_{}_value", index).as_str(),
            );

            context.build_store(context.function().stack[binding.as_str()], value);
        }

        Ok(())
    }
}
