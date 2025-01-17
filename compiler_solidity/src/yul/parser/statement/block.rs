//!
//! The source code block.
//!

use crate::yul::lexer::lexeme::symbol::Symbol;
use crate::yul::lexer::lexeme::Lexeme;
use crate::yul::lexer::Lexer;
use crate::yul::parser::statement::assignment::Assignment;
use crate::yul::parser::statement::expression::Expression;
use crate::yul::parser::statement::Statement;

///
/// The source code block.
///
#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    /// The block statements.
    pub statements: Vec<Statement>,
}

impl Block {
    ///
    /// The element parser.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Lexeme>) -> anyhow::Result<Self> {
        let lexeme = crate::yul::parser::take_or_next(initial, lexer)?;

        let mut statements = Vec::new();

        match lexeme {
            Lexeme::Symbol(Symbol::BracketCurlyLeft) => {}
            lexeme => anyhow::bail!("Expected one of {:?}, found `{}`", ["{"], lexeme),
        }

        let mut remaining = None;

        loop {
            match crate::yul::parser::take_or_next(remaining.take(), lexer)? {
                lexeme @ Lexeme::Keyword(_) => {
                    let (statement, next) = Statement::parse(lexer, Some(lexeme))?;
                    remaining = next;
                    statements.push(statement);
                }
                lexeme @ Lexeme::Literal(_) => {
                    statements
                        .push(Expression::parse(lexer, Some(lexeme)).map(Statement::Expression)?);
                }
                lexeme @ Lexeme::Identifier(_) => match lexer.peek()? {
                    Lexeme::Symbol(Symbol::Assignment) => {
                        statements.push(
                            Assignment::parse(lexer, Some(lexeme)).map(Statement::Assignment)?,
                        );
                    }
                    Lexeme::Symbol(Symbol::Comma) => {
                        statements.push(
                            Assignment::parse(lexer, Some(lexeme)).map(Statement::Assignment)?,
                        );
                    }
                    _ => {
                        statements.push(
                            Expression::parse(lexer, Some(lexeme)).map(Statement::Expression)?,
                        );
                    }
                },
                lexeme @ Lexeme::Symbol(Symbol::BracketCurlyLeft) => {
                    statements.push(Block::parse(lexer, Some(lexeme)).map(Statement::Block)?)
                }
                Lexeme::Symbol(Symbol::BracketCurlyRight) => break,
                lexeme => {
                    anyhow::bail!(
                        "Expected one of {:?}, found `{}`",
                        ["{keyword}", "{expression}", "{identifier}", "{", "}"],
                        lexeme
                    );
                }
            }
        }

        Ok(Self { statements })
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for Block
where
    D: compiler_llvm_context::Dependency,
{
    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        let current_function = context.function().to_owned();
        let current_block = context.basic_block();

        let mut functions = Vec::with_capacity(self.statements.len());
        let mut local_statements = Vec::with_capacity(self.statements.len());

        for statement in self.statements.into_iter() {
            match statement {
                Statement::FunctionDefinition(mut statement) => {
                    statement.declare(context)?;
                    functions.push(statement);
                }
                statement => local_statements.push(statement),
            }
        }

        for function in functions.into_iter() {
            function.into_llvm(context)?;
        }

        context.set_function(current_function.clone());
        context.set_basic_block(current_block);
        for statement in local_statements.into_iter() {
            if context.basic_block().get_terminator().is_some() {
                break;
            }

            match statement {
                Statement::Block(block) => {
                    block.into_llvm(context)?;
                }
                Statement::Expression(expression) => {
                    expression.into_llvm(context)?;
                }
                Statement::VariableDeclaration(statement) => statement.into_llvm(context)?,
                Statement::Assignment(statement) => statement.into_llvm(context)?,
                Statement::IfConditional(statement) => statement.into_llvm(context)?,
                Statement::Switch(statement) => statement.into_llvm(context)?,
                Statement::ForLoop(statement) => statement.into_llvm(context)?,
                Statement::Continue => {
                    context.build_unconditional_branch(context.r#loop().continue_block);
                    break;
                }
                Statement::Break => {
                    context.build_unconditional_branch(context.r#loop().join_block);
                    break;
                }
                Statement::Leave => {
                    context.build_unconditional_branch(context.function().return_block);
                    break;
                }
                statement => anyhow::bail!("Unexpected local statement: {:?}", statement),
            }
        }

        Ok(())
    }
}
