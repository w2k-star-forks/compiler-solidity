//!
//! The function call subexpression.
//!

pub mod name;

use inkwell::values::BasicValue;

use crate::generator::llvm::Context as LLVMContext;
use crate::lexer::lexeme::symbol::Symbol;
use crate::lexer::lexeme::Lexeme;
use crate::lexer::Lexer;
use crate::parser::block::statement::expression::Expression;

use self::name::Name;

///
/// The function call subexpression.
///
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    /// The function name.
    pub name: Name,
    /// The function arguments expression list.
    pub arguments: Vec<Expression>,
}

impl FunctionCall {
    ///
    /// The element parser, which acts like a constructor.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Lexeme>) -> Self {
        let lexeme = initial.unwrap_or_else(|| lexer.next());

        let name = match lexeme {
            Lexeme::Identifier(identifier) => Name::from(identifier.as_str()),
            lexeme => panic!("Expected an identifier, found {}", lexeme),
        };

        let mut arguments = Vec::new();
        loop {
            let argument = match lexer.next() {
                Lexeme::Symbol(Symbol::ParenthesisRight) => break,
                lexeme => Expression::parse(lexer, Some(lexeme)),
            };

            arguments.push(argument);

            match lexer.peek() {
                Lexeme::Symbol(Symbol::Comma) => {
                    lexer.next();
                    continue;
                }
                Lexeme::Symbol(Symbol::ParenthesisRight) => {
                    lexer.next();
                    break;
                }
                _ => break,
            }
        }

        Self { name, arguments }
    }

    ///
    /// Converts the function call into an LLVM value.
    ///
    pub fn into_llvm<'ctx>(
        mut self,
        context: &LLVMContext<'ctx>,
    ) -> Option<inkwell::values::BasicValueEnum<'ctx>> {
        match self.name {
            Name::Add => {
                let value = context.builder.build_int_add(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Sub => {
                let value = context.builder.build_int_sub(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Mul => {
                let value = context.builder.build_int_mul(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Div => {
                let value = context.builder.build_int_unsigned_div(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Mod => {
                let value = context.builder.build_int_unsigned_rem(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Lt => {
                let value = context.builder.build_int_compare(
                    inkwell::IntPredicate::ULT,
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                let value = context.builder.build_int_cast(
                    value,
                    context.integer_type(crate::BITLENGTH_DEFAULT),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Gt => {
                let value = context.builder.build_int_compare(
                    inkwell::IntPredicate::UGT,
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                let value = context.builder.build_int_cast(
                    value,
                    context.integer_type(crate::BITLENGTH_DEFAULT),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Eq => {
                let value = context.builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                let value = context.builder.build_int_cast(
                    value,
                    context.integer_type(crate::BITLENGTH_DEFAULT),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::And => {
                let value = context.builder.build_and(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Or => {
                let value = context.builder.build_or(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Xor => {
                let value = context.builder.build_xor(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::IsZero => {
                let value = context.builder.build_right_shift(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    context
                        .integer_type(crate::BITLENGTH_DEFAULT)
                        .const_int(0, false),
                    true,
                    "",
                );
                let value = context.builder.build_int_cast(
                    value,
                    context.integer_type(crate::BITLENGTH_DEFAULT),
                    "",
                );
                Some(value.as_basic_value_enum())
            }

            Name::Sdiv => {
                let value = context.builder.build_int_signed_div(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Smod => {
                let value = context.builder.build_int_signed_rem(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Slt => {
                let value = context.builder.build_int_compare(
                    inkwell::IntPredicate::SLT,
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                let value = context.builder.build_int_cast(
                    value,
                    context.integer_type(crate::BITLENGTH_DEFAULT),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Sgt => {
                let value = context.builder.build_int_compare(
                    inkwell::IntPredicate::SGT,
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                let value = context.builder.build_int_cast(
                    value,
                    context.integer_type(crate::BITLENGTH_DEFAULT),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Shl => {
                let value = context.builder.build_left_shift(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Shr => {
                let value = context.builder.build_right_shift(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    false,
                    "",
                );
                Some(value.as_basic_value_enum())
            }
            Name::Sar => {
                let value = context.builder.build_right_shift(
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    self.arguments
                        .remove(0)
                        .into_llvm(context)
                        .expect("Always exists")
                        .into_int_value(),
                    true,
                    "",
                );
                Some(value.as_basic_value_enum())
            }

            Name::UserDefined(name) => {
                let arguments: Vec<inkwell::values::BasicValueEnum> = self
                    .arguments
                    .into_iter()
                    .filter_map(|argument| argument.into_llvm(context))
                    .collect();
                let function = context
                    .module
                    .get_function(name.as_str())
                    .unwrap_or_else(|| panic!("Undeclared function {}", name));
                let return_value = context
                    .builder
                    .build_call(function, &arguments, "")
                    .try_as_basic_value();
                return_value.left()
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ok_compile_void() {
        let input = r#"{
            function bar() {}

            function foo() -> x {
                x := 42
                bar()
            }
        }"#;

        crate::compile(input);
    }

    #[test]
    fn ok_compile_non_void() {
        let input = r#"{
            function bar() -> x {
                x:= 42
            }

            function foo() -> x {
                x := bar()
            }
        }"#;

        crate::compile(input);
    }

    #[test]
    fn ok_compile_with_arguments() {
        let input = r#"{
            function foo(z) -> x {
                let y := 3
                x := add(3, y)
            }
        }"#;

        crate::compile(input);
    }

    #[test]
    fn ok_compile_builtin_add() {
        let input = r#"{
            function foo() -> x {let y := 3 x := add(3, y)}
        }"#;

        crate::compile(input);
    }

    #[test]
    fn ok_compile_builtin_sub() {
        let input = r#"{
            function foo() -> x {let y := 3 x := sub(3, y)}
        }"#;

        crate::compile(input);
    }

    #[test]
    fn ok_compile_builtin_mul() {
        let input = r#"{
            function foo() -> x {let y := 3 x := mul(3, y)}
        }"#;

        crate::compile(input);
    }

    #[test]
    fn ok_compile_builtin_div() {
        let input = r#"{
            function foo() -> x {let y := 3 x := div(3, y)}
        }"#;

        crate::compile(input);
    }

    #[test]
    fn ok_compile_builtin_sdiv() {
        let input = r#"{
            function foo() -> x {let y := 3 x := sdiv(3, y)}
        }"#;

        crate::compile(input);
    }

    #[test]
    fn ok_compile_builtin_mod() {
        let input = r#"{
            function foo() -> x {let y := 3 x := mod(3, y)}
        }"#;

        crate::compile(input);
    }

    #[test]
    fn ok_compile_builtin_smod() {
        let input = r#"{
            function foo() -> x {let y := 3 x := smod(3, y)}
        }"#;

        crate::compile(input);
    }
}
