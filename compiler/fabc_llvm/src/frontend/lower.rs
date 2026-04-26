use std::collections::BTreeMap;

use fabc_parser::ast::{
    decl::quote::QuoteDecl,
    expr::{
        literal::Literal as ParserLiteral, primitive::Primitive,
        BinaryOperator as ParserBinaryOperator, Expr as ParserExpr, Primary,
        UnaryOperator as ParserUnaryOperator,
    },
    init::{
        story::{
            part::{element::Element as StoryElement, Part as StoryPart},
            StoryInit,
        },
        Init,
    },
    stmt::{
        block::BlockStmt, expr::ExprStmt, goto::GotoStmt, r#if::ElseClause, r#if::IfStmt,
        r#let::LetStmt, r#return::ReturnStmt, Stmt as ParserStmt,
    },
};

use crate::{
    error::{Error, Result},
    ir::{
        BinaryOperator, Block, DialogueSpec, Expr, FunctionSpec, Literal, MemberSegment, PartSpec,
        QuoteSpec, SelectionSpec, StepSpec, Stmt, StoryProgram, UnaryOperator,
    },
};

#[derive(Default)]
pub struct Lowerer {
    functions: Vec<FunctionSpec>,
    part_names: Vec<String>,
}

impl Lowerer {
    pub fn lower_inits(mut self, inits: Vec<Init>) -> Result<StoryProgram> {
        let story_inits: Vec<&StoryInit> = inits
            .iter()
            .filter_map(|init| match init {
                Init::Story(story) => Some(story),
                Init::Module(module) => {
                    let _ = module;
                    None
                }
            })
            .collect();

        for init in &inits {
            if let Init::Module(module) = init {
                return Err(Error::UnsupportedModuleImport(module.path.clone()));
            }
        }

        let story = match story_inits.as_slice() {
            [] => return Err(Error::MissingStoryInit),
            [story] => *story,
            many => return Err(Error::MultipleStoryInits(many.len())),
        };

        self.part_names = story.parts.iter().map(|part| part.ident.clone()).collect();

        let metadata = match &story.metadata {
            Some(metadata) => self.lower_object_map(&metadata.object.map)?,
            None => BTreeMap::new(),
        };
        let start_part = self.extract_start_part(&metadata)?;

        let mut parts = Vec::with_capacity(story.parts.len());
        for part in &story.parts {
            parts.push(self.lower_part(part)?);
        }

        Ok(StoryProgram {
            start_part,
            metadata,
            parts,
            functions: self.functions,
        })
    }

    fn extract_start_part(&self, metadata: &BTreeMap<String, Expr>) -> Result<String> {
        let Some(start_expr) = metadata.get("start") else {
            return Err(Error::MissingStartPart);
        };

        let start = match start_expr {
            Expr::Literal(Literal::String(value)) => value.clone(),
            Expr::StoryReference(value) => value.clone(),
            Expr::Identifier(value) if self.is_known_part(value) => value.clone(),
            _ => return Err(Error::InvalidStartMetadata),
        };

        if !self.is_known_part(&start) {
            return Err(Error::UnknownPart(start));
        }

        Ok(start)
    }

    fn lower_part(&mut self, part: &StoryPart) -> Result<PartSpec> {
        let mut steps = Vec::new();

        for element in &part.elements {
            match element {
                StoryElement::Narration(narration) => {
                    steps.push(StepSpec::Narration(self.lower_quote(&narration.quote)?));
                }
                StoryElement::Dialogue(dialogue) => {
                    for quote in &dialogue.quotes {
                        steps.push(StepSpec::Dialogue(DialogueSpec {
                            speaker: dialogue.speaker.clone(),
                            quote: self.lower_quote(quote)?,
                        }));
                    }
                }
                StoryElement::Selection(selection) => {
                    let mut choices = Vec::with_capacity(selection.choices.len());
                    for choice in &selection.choices {
                        choices.push(self.lower_quote(choice)?);
                    }

                    steps.push(StepSpec::Selection(SelectionSpec { choices }));
                }
            }
        }

        Ok(PartSpec {
            id: part.ident.clone(),
            steps,
        })
    }

    fn lower_quote(&mut self, quote: &QuoteDecl) -> Result<QuoteSpec> {
        let mut properties = BTreeMap::new();
        let mut next_action = None;

        if let Some(object) = &quote.properties {
            for (key, value) in &object.map {
                let lowered = self.lower_expr(value)?;
                if key == "next" {
                    match lowered {
                        Expr::Closure(function_id) => next_action = Some(function_id),
                        _ => return Err(Error::InvalidNextHandler),
                    }
                } else {
                    properties.insert(key.clone(), lowered);
                }
            }
        }

        Ok(QuoteSpec {
            node_id: quote.info.id,
            text: quote.text.clone(),
            properties,
            next_action,
        })
    }

    fn lower_object_map(
        &mut self,
        map: &BTreeMap<String, ParserExpr>,
    ) -> Result<BTreeMap<String, Expr>> {
        let mut lowered = BTreeMap::new();
        for (key, value) in map {
            lowered.insert(key.clone(), self.lower_expr(value)?);
        }
        Ok(lowered)
    }

    fn lower_block(&mut self, block: &BlockStmt) -> Result<Block> {
        let mut statements = Vec::with_capacity(block.statements.len());
        for statement in &block.statements {
            statements.push(self.lower_stmt(statement)?);
        }
        Ok(Block { statements })
    }

    fn lower_stmt(&mut self, statement: &ParserStmt) -> Result<Stmt> {
        Ok(match statement {
            ParserStmt::Expr(ExprStmt { expr, .. }) => Stmt::Expr(self.lower_expr(expr)?),
            ParserStmt::Block(block) => Stmt::Block(self.lower_block(block)?),
            ParserStmt::Let(LetStmt {
                name, initializer, ..
            }) => Stmt::Let {
                name: name.clone(),
                initializer: self.lower_expr(initializer)?,
            },
            ParserStmt::Goto(GotoStmt { target, .. }) => {
                Stmt::Goto(self.lower_goto_target(target)?)
            }
            ParserStmt::If(if_stmt) => self.lower_if(if_stmt)?,
            ParserStmt::Return(ReturnStmt { value, .. }) => Stmt::Return(
                value
                    .as_ref()
                    .map(|expr| self.lower_expr(expr))
                    .transpose()?,
            ),
        })
    }

    fn lower_if(&mut self, if_stmt: &IfStmt) -> Result<Stmt> {
        Ok(Stmt::If {
            condition: self.lower_expr(&if_stmt.condition)?,
            then_branch: self.lower_block(&if_stmt.then_branch)?,
            else_branch: match &if_stmt.else_branch {
                Some(ElseClause::If(nested_if)) => Some(Box::new(self.lower_if(nested_if)?)),
                Some(ElseClause::Block(block)) => {
                    Some(Box::new(Stmt::Block(self.lower_block(block)?)))
                }
                None => None,
            },
        })
    }

    fn lower_goto_target(&mut self, expr: &ParserExpr) -> Result<Expr> {
        if let Some(part_name) = self.extract_part_reference(expr) {
            return Ok(Expr::StoryReference(part_name));
        }

        self.lower_expr(expr)
    }

    fn extract_part_reference(&self, expr: &ParserExpr) -> Option<String> {
        match expr {
            ParserExpr::Primary {
                value: Primary::Primitive(Primitive::StoryIdentifier { name, .. }),
                ..
            } => Some(name.clone()),
            ParserExpr::Primary {
                value: Primary::Primitive(Primitive::Identifier { name, .. }),
                ..
            } if self.is_known_part(name) => Some(name.clone()),
            ParserExpr::Primary {
                value: Primary::Literal(ParserLiteral::String { value, .. }),
                ..
            } if self.is_known_part(value) => Some(value.clone()),
            _ => None,
        }
    }

    fn lower_expr(&mut self, expr: &ParserExpr) -> Result<Expr> {
        Ok(match expr {
            ParserExpr::Binary {
                left,
                operator,
                right,
                ..
            } => Expr::Binary {
                left: Box::new(self.lower_expr(left)?),
                operator: (*operator).into(),
                right: Box::new(self.lower_expr(right)?),
            },
            ParserExpr::Unary {
                operator, right, ..
            } => Expr::Unary {
                operator: (*operator).into(),
                right: Box::new(self.lower_expr(right)?),
            },
            ParserExpr::Assignment { name, value, .. } => Expr::Assignment {
                target: Box::new(self.lower_expr(name)?),
                value: Box::new(self.lower_expr(value)?),
            },
            ParserExpr::MemberAccess { left, members, .. } => Expr::MemberAccess {
                base: Box::new(self.lower_expr(left)?),
                members: members
                    .iter()
                    .map(|member| self.lower_member_segment(member))
                    .collect::<Result<Vec<_>>>()?,
            },
            ParserExpr::Call {
                callee, arguments, ..
            } => Expr::Call {
                callee: Box::new(self.lower_expr(callee)?),
                arguments: arguments
                    .iter()
                    .map(|argument| self.lower_expr(argument))
                    .collect::<Result<Vec<_>>>()?,
            },
            ParserExpr::Primary { value, .. } => self.lower_primary(value)?,
            ParserExpr::Grouping { expression, .. } => {
                Expr::Grouping(Box::new(self.lower_expr(expression)?))
            }
        })
    }

    fn lower_primary(&mut self, primary: &Primary) -> Result<Expr> {
        Ok(match primary {
            Primary::Literal(ParserLiteral::Number { value, .. }) => {
                Expr::Literal(Literal::Number(*value))
            }
            Primary::Literal(ParserLiteral::Boolean { value, .. }) => {
                Expr::Literal(Literal::Boolean(*value))
            }
            Primary::Literal(ParserLiteral::String { value, .. }) => {
                Expr::Literal(Literal::String(value.clone()))
            }
            Primary::Literal(ParserLiteral::None { .. }) => Expr::Literal(Literal::None),
            Primary::Primitive(Primitive::Identifier { name, .. }) => {
                Expr::Identifier(name.clone())
            }
            Primary::Primitive(Primitive::StoryIdentifier { name, .. }) => {
                Expr::StoryReference(name.clone())
            }
            Primary::Primitive(Primitive::Context { .. }) => Expr::Context,
            Primary::Primitive(Primitive::Grouping { expr, .. }) => {
                Expr::Grouping(Box::new(self.lower_expr(expr)?))
            }
            Primary::Primitive(Primitive::Object { value, .. }) => {
                Expr::Object(self.lower_object_map(&value.map)?)
            }
            Primary::Primitive(Primitive::Closure { info, params, body }) => {
                let mut lowered_params = Vec::with_capacity(params.len());
                for param in params {
                    match param {
                        Primitive::Identifier { name, .. } => lowered_params.push(name.clone()),
                        _ => return Err(Error::InvalidClosureParameter),
                    }
                }

                let function_id = self.functions.len();
                let body = self.lower_block(body)?;
                self.functions.push(FunctionSpec {
                    id: function_id,
                    node_id: info.id,
                    params: lowered_params,
                    body,
                });

                Expr::Closure(function_id)
            }
        })
    }

    fn lower_member_segment(&mut self, member: &ParserExpr) -> Result<MemberSegment> {
        match member {
            ParserExpr::Primary {
                value: Primary::Primitive(Primitive::Identifier { name, .. }),
                ..
            }
            | ParserExpr::Primary {
                value: Primary::Primitive(Primitive::StoryIdentifier { name, .. }),
                ..
            } => Ok(MemberSegment::Key(name.clone())),
            ParserExpr::Primary {
                value: Primary::Literal(ParserLiteral::String { value, .. }),
                ..
            } => Ok(MemberSegment::Key(value.clone())),
            _ => Ok(MemberSegment::Expr(Box::new(self.lower_expr(member)?))),
        }
    }

    fn is_known_part(&self, name: &str) -> bool {
        self.part_names.iter().any(|part| part == name)
    }
}

impl From<ParserBinaryOperator> for BinaryOperator {
    fn from(value: ParserBinaryOperator) -> Self {
        match value {
            ParserBinaryOperator::EqualEqual => BinaryOperator::EqualEqual,
            ParserBinaryOperator::NotEqual => BinaryOperator::NotEqual,
            ParserBinaryOperator::Greater => BinaryOperator::Greater,
            ParserBinaryOperator::GreaterEqual => BinaryOperator::GreaterEqual,
            ParserBinaryOperator::Less => BinaryOperator::Less,
            ParserBinaryOperator::LessEqual => BinaryOperator::LessEqual,
            ParserBinaryOperator::Add => BinaryOperator::Add,
            ParserBinaryOperator::Subtraction => BinaryOperator::Subtract,
            ParserBinaryOperator::Multiply => BinaryOperator::Multiply,
            ParserBinaryOperator::Divide => BinaryOperator::Divide,
            ParserBinaryOperator::And => BinaryOperator::And,
            ParserBinaryOperator::Or => BinaryOperator::Or,
        }
    }
}

impl From<ParserUnaryOperator> for UnaryOperator {
    fn from(value: ParserUnaryOperator) -> Self {
        match value {
            ParserUnaryOperator::Not => UnaryOperator::Not,
            ParserUnaryOperator::Negate => UnaryOperator::Negate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lowerer;
    use fabc_parser::Parser;

    #[test]
    fn lowers_story_into_flat_steps() {
        let parsed = Parser::parse_str(
            r#"
            Story { start: "intro" }

            # intro
            [guide]
            > "Welcome."
            > "Choose carefully."
            - "Go left." { next: () => { goto end_left; } }
            - "Go right." { next: () => { goto end_right; } }

            # end_left
            * "Left"

            # end_right
            * "Right"
            "#,
        );

        let program = Lowerer::default()
            .lower_inits(parsed.result)
            .expect("lowering should succeed");

        assert_eq!(program.start_part, "intro");
        assert_eq!(program.parts[0].steps.len(), 3);
        assert_eq!(program.functions.len(), 2);
    }
}
