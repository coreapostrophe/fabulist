use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use fabc_parser::{
    ast::{
        decl::{object::ObjectDecl, quote::QuoteDecl},
        expr::{literal::Literal, primitive::Primitive, Expr, Primary},
        init::{
            module::ModuleInit,
            story::{
                metadata::Metadata,
                part::{
                    element::{
                        dialogue::DialogueElement, narration::NarrationElement,
                        selection::SelectionElement, Element,
                    },
                    Part,
                },
                StoryInit,
            },
            Init,
        },
        stmt::{
            block::BlockStmt,
            expr::ExprStmt,
            goto::GotoStmt,
            r#if::{ElseClause, IfStmt},
            r#let::LetStmt,
            r#return::ReturnStmt,
            Stmt,
        },
        NodeInfo,
    },
    Parser,
};

use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq)]
enum ExportValue {
    StoryTarget(String),
    Number(f64),
    Boolean(bool),
    String(String),
    None,
}

#[derive(Default)]
pub(crate) struct ModuleLinker {
    part_origins: BTreeMap<String, PathBuf>,
}

struct LinkedStory {
    info: NodeInfo,
    metadata: Option<Metadata>,
    parts: Vec<Part>,
    exports: BTreeMap<String, ExportValue>,
}

struct ExportContext<'a> {
    path: &'a Path,
    namespace: Option<&'a str>,
    local_parts: &'a BTreeSet<String>,
    aliases: &'a BTreeMap<String, String>,
    imported_exports: &'a BTreeMap<String, BTreeMap<String, ExportValue>>,
}

impl ModuleLinker {
    pub(crate) fn link_inits(mut self, entry: &Path) -> Result<Vec<Init>> {
        let mut stack = Vec::new();
        let linked = self.load_file(entry, None, true, &mut stack)?;

        Ok(vec![Init::Story(StoryInit {
            info: linked.info,
            metadata: linked.metadata,
            parts: linked.parts,
        })])
    }

    fn load_file(
        &mut self,
        path: &Path,
        namespace: Option<&str>,
        keep_metadata: bool,
        stack: &mut Vec<PathBuf>,
    ) -> Result<LinkedStory> {
        let canonical = fs::canonicalize(path).map_err(|source| Error::Io {
            path: path.to_path_buf(),
            source,
        })?;

        if let Some(start) = stack.iter().position(|current| current == &canonical) {
            let mut chain = stack[start..].to_vec();
            chain.push(canonical.clone());
            return Err(Error::CircularImport { chain });
        }

        let source = fs::read_to_string(&canonical).map_err(|source| Error::Io {
            path: canonical.clone(),
            source,
        })?;

        let parsed = Parser::parse_str(&source);
        if !parsed.errors.is_empty() {
            return Err(Error::ParseDiagnostics {
                path: canonical.clone(),
                diagnostics: parsed.errors,
            });
        }

        let (mut story, modules) = self.split_inits(&canonical, parsed.result)?;
        let local_parts: BTreeSet<String> =
            story.parts.iter().map(|part| part.ident.clone()).collect();

        stack.push(canonical.clone());

        let mut imported_parts = Vec::new();
        let mut aliases = BTreeMap::new();
        let mut imported_exports = BTreeMap::new();

        for module in modules {
            let resolved_path = self.resolve_import_path(&canonical, &module.path)?;
            let child_namespace = module
                .alias
                .as_deref()
                .map(|alias| qualify_namespace(namespace, alias))
                .or_else(|| namespace.map(ToOwned::to_owned));

            if let (Some(alias), Some(prefix)) = (module.alias.as_ref(), child_namespace.as_ref()) {
                aliases.insert(alias.clone(), prefix.clone());
            }

            let linked =
                self.load_file(&resolved_path, child_namespace.as_deref(), false, stack)?;
            if let Some(alias) = module.alias.as_ref() {
                imported_exports.insert(alias.clone(), linked.exports);
            }
            imported_parts.extend(linked.parts);
        }

        stack.pop();

        if let Some(metadata) = story.metadata.as_mut() {
            self.rewrite_metadata(
                metadata,
                namespace,
                &local_parts,
                &aliases,
                &imported_exports,
            );
        }

        let exports = self.extract_exports(
            &canonical,
            story.metadata.as_ref(),
            namespace,
            &local_parts,
            &aliases,
            &imported_exports,
        )?;

        if let Some(metadata) = story.metadata.as_mut() {
            metadata.object.map.remove("exports");
        }

        if !keep_metadata {
            story.metadata = None;
        }

        let mut local_story_parts = Vec::with_capacity(story.parts.len());
        for mut part in story.parts {
            self.rewrite_part(
                &mut part,
                namespace,
                &local_parts,
                &aliases,
                &imported_exports,
            );
            part.ident = qualify_namespace(namespace, &part.ident);
            self.register_part(&part.ident, &canonical)?;
            local_story_parts.push(part);
        }

        let mut parts = imported_parts;
        parts.extend(local_story_parts);

        Ok(LinkedStory {
            info: story.info,
            metadata: story.metadata,
            parts,
            exports,
        })
    }

    fn split_inits(&self, path: &Path, inits: Vec<Init>) -> Result<(StoryInit, Vec<ModuleInit>)> {
        let mut modules = Vec::new();
        let mut stories = Vec::new();

        for init in inits {
            match init {
                Init::Story(story) => stories.push(story),
                Init::Module(module) => modules.push(module),
            }
        }

        match stories.len() {
            0 => Err(Error::MissingStory {
                path: path.to_path_buf(),
            }),
            1 => Ok((stories.pop().expect("story exists"), modules)),
            count => Err(Error::MultipleStories {
                path: path.to_path_buf(),
                count,
            }),
        }
    }

    fn resolve_import_path(&self, source_path: &Path, import: &str) -> Result<PathBuf> {
        let import_path = PathBuf::from(import);
        if import_path.is_absolute() {
            return Ok(import_path);
        }

        source_path
            .parent()
            .map(|parent| parent.join(&import_path))
            .ok_or_else(|| Error::InvalidImportPath {
                from: source_path.to_path_buf(),
                import: import.to_string(),
            })
    }

    fn register_part(&mut self, part: &str, path: &Path) -> Result<()> {
        if let Some(first) = self.part_origins.get(part) {
            return Err(Error::DuplicatePart {
                part: part.to_string(),
                first: first.clone(),
                second: path.to_path_buf(),
            });
        }

        self.part_origins
            .insert(part.to_string(), path.to_path_buf());
        Ok(())
    }

    fn rewrite_metadata(
        &self,
        metadata: &mut Metadata,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) {
        for (key, value) in &mut metadata.object.map {
            if key == "start" {
                self.rewrite_story_target_expr(
                    value,
                    namespace,
                    local_parts,
                    aliases,
                    imported_exports,
                );
            } else {
                self.rewrite_expr(value, namespace, local_parts, aliases, imported_exports);
            }
        }
    }

    fn rewrite_part(
        &self,
        part: &mut Part,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) {
        for element in &mut part.elements {
            match element {
                Element::Dialogue(DialogueElement { quotes, .. }) => {
                    for quote in quotes {
                        self.rewrite_quote(
                            quote,
                            namespace,
                            local_parts,
                            aliases,
                            imported_exports,
                        );
                    }
                }
                Element::Narration(NarrationElement { quote, .. }) => {
                    self.rewrite_quote(quote, namespace, local_parts, aliases, imported_exports);
                }
                Element::Selection(SelectionElement { choices, .. }) => {
                    for quote in choices {
                        self.rewrite_quote(
                            quote,
                            namespace,
                            local_parts,
                            aliases,
                            imported_exports,
                        );
                    }
                }
            }
        }
    }

    fn rewrite_quote(
        &self,
        quote: &mut QuoteDecl,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) {
        let Some(properties) = quote.properties.as_mut() else {
            return;
        };

        for value in properties.map.values_mut() {
            self.rewrite_expr(value, namespace, local_parts, aliases, imported_exports);
        }
    }

    fn rewrite_block(
        &self,
        block: &mut BlockStmt,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) {
        for statement in &mut block.statements {
            self.rewrite_stmt(statement, namespace, local_parts, aliases, imported_exports);
        }
    }

    fn rewrite_stmt(
        &self,
        statement: &mut Stmt,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) {
        match statement {
            Stmt::Expr(ExprStmt { expr, .. }) => {
                self.rewrite_expr(expr, namespace, local_parts, aliases, imported_exports);
            }
            Stmt::Block(block) => {
                self.rewrite_block(block, namespace, local_parts, aliases, imported_exports);
            }
            Stmt::Let(LetStmt { initializer, .. }) => {
                self.rewrite_expr(
                    initializer,
                    namespace,
                    local_parts,
                    aliases,
                    imported_exports,
                );
            }
            Stmt::Goto(GotoStmt { target, .. }) => {
                self.rewrite_story_target_expr(
                    target,
                    namespace,
                    local_parts,
                    aliases,
                    imported_exports,
                );
            }
            Stmt::If(if_stmt) => {
                self.rewrite_if_stmt(if_stmt, namespace, local_parts, aliases, imported_exports);
            }
            Stmt::Return(ReturnStmt { value, .. }) => {
                if let Some(value) = value {
                    self.rewrite_expr(value, namespace, local_parts, aliases, imported_exports);
                }
            }
        }
    }

    fn rewrite_if_stmt(
        &self,
        if_stmt: &mut IfStmt,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) {
        self.rewrite_expr(
            &mut if_stmt.condition,
            namespace,
            local_parts,
            aliases,
            imported_exports,
        );
        self.rewrite_block(
            &mut if_stmt.then_branch,
            namespace,
            local_parts,
            aliases,
            imported_exports,
        );

        if let Some(else_branch) = &mut if_stmt.else_branch {
            match else_branch {
                ElseClause::If(nested_if) => {
                    self.rewrite_if_stmt(
                        nested_if,
                        namespace,
                        local_parts,
                        aliases,
                        imported_exports,
                    );
                }
                ElseClause::Block(block) => {
                    self.rewrite_block(block, namespace, local_parts, aliases, imported_exports);
                }
            }
        }
    }

    fn rewrite_expr(
        &self,
        expr: &mut Expr,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) {
        if let Some(replacement) = self.resolve_module_member_expr(expr, aliases, imported_exports)
        {
            *expr = replacement;
            return;
        }

        match expr {
            Expr::Binary { left, right, .. } => {
                self.rewrite_expr(left, namespace, local_parts, aliases, imported_exports);
                self.rewrite_expr(right, namespace, local_parts, aliases, imported_exports);
            }
            Expr::Unary { right, .. } => {
                self.rewrite_expr(right, namespace, local_parts, aliases, imported_exports);
            }
            Expr::Assignment { name, value, .. } => {
                self.rewrite_expr(name, namespace, local_parts, aliases, imported_exports);
                self.rewrite_expr(value, namespace, local_parts, aliases, imported_exports);
            }
            Expr::MemberAccess { left, members, .. } => {
                self.rewrite_expr(left, namespace, local_parts, aliases, imported_exports);
                for member in members {
                    self.rewrite_expr(member, namespace, local_parts, aliases, imported_exports);
                }
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                self.rewrite_expr(callee, namespace, local_parts, aliases, imported_exports);
                for argument in arguments {
                    self.rewrite_expr(argument, namespace, local_parts, aliases, imported_exports);
                }
            }
            Expr::Primary { value, .. } => match value {
                Primary::Literal(_) => {}
                Primary::Primitive(Primitive::Grouping { expr, .. }) => {
                    self.rewrite_expr(expr, namespace, local_parts, aliases, imported_exports);
                }
                Primary::Primitive(Primitive::Object { value, .. }) => {
                    self.rewrite_object(value, namespace, local_parts, aliases, imported_exports);
                }
                Primary::Primitive(Primitive::Closure { body, .. }) => {
                    self.rewrite_block(body, namespace, local_parts, aliases, imported_exports);
                }
                Primary::Primitive(Primitive::Identifier { .. })
                | Primary::Primitive(Primitive::StoryIdentifier { .. })
                | Primary::Primitive(Primitive::Context { .. }) => {}
            },
            Expr::Grouping { expression, .. } => {
                self.rewrite_expr(
                    expression,
                    namespace,
                    local_parts,
                    aliases,
                    imported_exports,
                );
            }
        }
    }

    fn rewrite_object(
        &self,
        object: &mut ObjectDecl,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) {
        for value in object.map.values_mut() {
            self.rewrite_expr(value, namespace, local_parts, aliases, imported_exports);
        }
    }

    fn rewrite_story_target_expr(
        &self,
        expr: &mut Expr,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) {
        if let Some(replacement) = self.resolve_module_member_expr(expr, aliases, imported_exports)
        {
            *expr = replacement;
        }

        if let Some(target) = resolve_story_target(expr, namespace, local_parts) {
            *expr = make_string_expr(expr.info(), target);
        } else {
            self.rewrite_expr(expr, namespace, local_parts, aliases, imported_exports);
        }
    }

    fn extract_exports(
        &self,
        path: &Path,
        metadata: Option<&Metadata>,
        namespace: Option<&str>,
        local_parts: &BTreeSet<String>,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) -> Result<BTreeMap<String, ExportValue>> {
        let Some(metadata) = metadata else {
            return Ok(BTreeMap::new());
        };

        let Some(exports_expr) = metadata.object.map.get("exports") else {
            return Ok(BTreeMap::new());
        };

        let exports_object = match exports_expr {
            Expr::Primary {
                value: Primary::Primitive(Primitive::Object { value, .. }),
                ..
            } => value,
            _ => {
                return Err(Error::InvalidExportsObject {
                    path: path.to_path_buf(),
                });
            }
        };

        let context = ExportContext {
            path,
            namespace,
            local_parts,
            aliases,
            imported_exports,
        };

        let mut exports = BTreeMap::new();
        for (name, value) in &exports_object.map {
            exports.insert(
                name.clone(),
                self.extract_export_value(name, value, &context)?,
            );
        }

        Ok(exports)
    }

    fn extract_export_value(
        &self,
        export_name: &str,
        expr: &Expr,
        context: &ExportContext<'_>,
    ) -> Result<ExportValue> {
        if let Some((alias, segments)) = module_member_segments(expr) {
            if let Some(exports) = context.imported_exports.get(&alias) {
                if segments.len() == 1 {
                    if let Some(value) = exports.get(&segments[0]) {
                        return Ok(value.clone());
                    }
                }
            }

            if let Some(prefix) = context.aliases.get(&alias) {
                let qualified = format!("{prefix}.{}", segments.join("."));
                if self.part_origins.contains_key(&qualified) {
                    return Ok(ExportValue::StoryTarget(qualified));
                }
            }
        }

        match expr {
            Expr::Primary {
                value: Primary::Primitive(Primitive::StoryIdentifier { name, .. }),
                ..
            } => Ok(ExportValue::StoryTarget(qualify_story_target(
                context.namespace,
                context.local_parts,
                name,
            ))),
            Expr::Primary {
                value: Primary::Primitive(Primitive::Identifier { name, .. }),
                ..
            } if context.local_parts.contains(name) => Ok(ExportValue::StoryTarget(
                qualify_namespace(context.namespace, name),
            )),
            Expr::Primary {
                value: Primary::Literal(Literal::Boolean { value, .. }),
                ..
            } => Ok(ExportValue::Boolean(*value)),
            Expr::Primary {
                value: Primary::Literal(Literal::String { value, .. }),
                ..
            } => Ok(ExportValue::String(value.clone())),
            Expr::Primary {
                value: Primary::Literal(Literal::Number { value, .. }),
                ..
            } => Ok(ExportValue::Number(*value)),
            Expr::Primary {
                value: Primary::Literal(Literal::None { .. }),
                ..
            } => Ok(ExportValue::None),
            _ => Err(Error::InvalidExportValue {
                path: context.path.to_path_buf(),
                export: export_name.to_string(),
            }),
        }
    }

    fn resolve_module_member_expr(
        &self,
        expr: &Expr,
        aliases: &BTreeMap<String, String>,
        imported_exports: &BTreeMap<String, BTreeMap<String, ExportValue>>,
    ) -> Option<Expr> {
        let (alias, segments) = module_member_segments(expr)?;

        if let Some(exports) = imported_exports.get(&alias) {
            if segments.len() == 1 {
                if let Some(value) = exports.get(&segments[0]) {
                    return Some(make_export_expr(expr.info(), value));
                }
            }
        }

        let prefix = aliases.get(&alias)?;
        let qualified = format!("{prefix}.{}", segments.join("."));
        if self.part_origins.contains_key(&qualified) {
            Some(make_story_reference_expr(expr.info(), qualified))
        } else {
            None
        }
    }
}

fn qualify_namespace(namespace: Option<&str>, name: &str) -> String {
    match namespace {
        Some(namespace) if !namespace.is_empty() => format!("{namespace}.{name}"),
        _ => name.to_string(),
    }
}

fn resolve_story_target(
    expr: &Expr,
    namespace: Option<&str>,
    local_parts: &BTreeSet<String>,
) -> Option<String> {
    match expr {
        Expr::Primary {
            value: Primary::Primitive(Primitive::StoryIdentifier { name, .. }),
            ..
        } => Some(name.clone()),
        Expr::Primary {
            value: Primary::Primitive(Primitive::Identifier { name, .. }),
            ..
        } if local_parts.contains(name) => Some(qualify_namespace(namespace, name)),
        Expr::Primary {
            value: Primary::Literal(Literal::String { value, .. }),
            ..
        } if local_parts.contains(value) => Some(qualify_namespace(namespace, value)),
        _ => None,
    }
}

fn qualify_story_target(
    namespace: Option<&str>,
    local_parts: &BTreeSet<String>,
    name: &str,
) -> String {
    if local_parts.contains(name) {
        qualify_namespace(namespace, name)
    } else {
        name.to_string()
    }
}

fn module_member_segments(expr: &Expr) -> Option<(String, Vec<String>)> {
    let Expr::MemberAccess { left, members, .. } = expr else {
        return None;
    };

    let alias = static_segment(left)?;
    let segments = members
        .iter()
        .map(static_segment)
        .collect::<Option<Vec<_>>>()?;

    if segments.is_empty() {
        None
    } else {
        Some((alias, segments))
    }
}

fn static_segment(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Primary {
            value: Primary::Primitive(Primitive::Identifier { name, .. }),
            ..
        }
        | Expr::Primary {
            value: Primary::Primitive(Primitive::StoryIdentifier { name, .. }),
            ..
        } => Some(name.clone()),
        Expr::Primary {
            value: Primary::Literal(Literal::String { value, .. }),
            ..
        } => Some(value.clone()),
        _ => None,
    }
}

fn make_string_expr(info: &NodeInfo, value: String) -> Expr {
    Expr::Primary {
        info: clone_info(info),
        value: Primary::Literal(Literal::String {
            info: clone_info(info),
            value,
        }),
    }
}

fn make_story_reference_expr(info: &NodeInfo, value: String) -> Expr {
    Expr::Primary {
        info: clone_info(info),
        value: Primary::Primitive(Primitive::StoryIdentifier {
            info: clone_info(info),
            name: value,
        }),
    }
}

fn make_export_expr(info: &NodeInfo, value: &ExportValue) -> Expr {
    match value {
        ExportValue::StoryTarget(value) => make_story_reference_expr(info, value.clone()),
        ExportValue::Number(value) => Expr::Primary {
            info: clone_info(info),
            value: Primary::Literal(Literal::Number {
                info: clone_info(info),
                value: *value,
            }),
        },
        ExportValue::Boolean(value) => Expr::Primary {
            info: clone_info(info),
            value: Primary::Literal(Literal::Boolean {
                info: clone_info(info),
                value: *value,
            }),
        },
        ExportValue::String(value) => make_string_expr(info, value.clone()),
        ExportValue::None => Expr::Primary {
            info: clone_info(info),
            value: Primary::Literal(Literal::None {
                info: clone_info(info),
            }),
        },
    }
}

fn clone_info(info: &NodeInfo) -> NodeInfo {
    NodeInfo {
        id: info.id,
        span: info.span.clone(),
    }
}
