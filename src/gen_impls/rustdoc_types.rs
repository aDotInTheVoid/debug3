// AUTOGENERATED FILE, DO NOT EDIT
//
// Crate Name: `rustdoc_types`
// Crate Version: `0.11.0`
impl crate::Debug for rustdoc_types::Abi {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Rust => {
                f.debug_tuple("Rust").finish();
            }
            Self::C { unwind } => f.debug_struct("C").field("unwind", unwind).finish(),
            Self::Cdecl { unwind } => f.debug_struct("Cdecl").field("unwind", unwind).finish(),
            Self::Stdcall { unwind } => f.debug_struct("Stdcall").field("unwind", unwind).finish(),
            Self::Fastcall { unwind } => {
                f.debug_struct("Fastcall").field("unwind", unwind).finish()
            }
            Self::Aapcs { unwind } => f.debug_struct("Aapcs").field("unwind", unwind).finish(),
            Self::Win64 { unwind } => f.debug_struct("Win64").field("unwind", unwind).finish(),
            Self::SysV64 { unwind } => f.debug_struct("SysV64").field("unwind", unwind).finish(),
            Self::System { unwind } => f.debug_struct("System").field("unwind", unwind).finish(),
            Self::Other(__0) => {
                f.debug_tuple("Other").field(__0).finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::Constant {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Constant")
            .field("type_", &self.type_)
            .field("expr", &self.expr)
            .field("value", &self.value)
            .field("is_literal", &self.is_literal)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Crate {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Crate")
            .field("root", &self.root)
            .field("crate_version", &self.crate_version)
            .field("includes_private", &self.includes_private)
            .field("index", &self.index)
            .field("paths", &self.paths)
            .field("external_crates", &self.external_crates)
            .field("format_version", &self.format_version)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Deprecation {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Deprecation")
            .field("since", &self.since)
            .field("note", &self.note)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Enum {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Enum")
            .field("generics", &self.generics)
            .field("variants_stripped", &self.variants_stripped)
            .field("variants", &self.variants)
            .field("impls", &self.impls)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::ExternalCrate {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("ExternalCrate")
            .field("name", &self.name)
            .field("html_root_url", &self.html_root_url)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::FnDecl {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("FnDecl")
            .field("inputs", &self.inputs)
            .field("output", &self.output)
            .field("c_variadic", &self.c_variadic)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Function {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Function")
            .field("decl", &self.decl)
            .field("generics", &self.generics)
            .field("header", &self.header)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::FunctionPointer {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("FunctionPointer")
            .field("decl", &self.decl)
            .field("generic_params", &self.generic_params)
            .field("header", &self.header)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::GenericArg {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Lifetime(__0) => {
                f.debug_tuple("Lifetime").field(__0).finish();
            }
            Self::Type(__0) => {
                f.debug_tuple("Type").field(__0).finish();
            }
            Self::Const(__0) => {
                f.debug_tuple("Const").field(__0).finish();
            }
            Self::Infer => {
                f.debug_tuple("Infer").finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::GenericArgs {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::AngleBracketed { args, bindings } => f
                .debug_struct("AngleBracketed")
                .field("args", args)
                .field("bindings", bindings)
                .finish(),
            Self::Parenthesized { inputs, output } => f
                .debug_struct("Parenthesized")
                .field("inputs", inputs)
                .field("output", output)
                .finish(),
        }
    }
}
impl crate::Debug for rustdoc_types::GenericBound {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::TraitBound {
                trait_,
                generic_params,
                modifier,
            } => f
                .debug_struct("TraitBound")
                .field("trait_", trait_)
                .field("generic_params", generic_params)
                .field("modifier", modifier)
                .finish(),
            Self::Outlives(__0) => {
                f.debug_tuple("Outlives").field(__0).finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::GenericParamDef {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("GenericParamDef")
            .field("name", &self.name)
            .field("kind", &self.kind)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::GenericParamDefKind {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Lifetime { outlives } => f
                .debug_struct("Lifetime")
                .field("outlives", outlives)
                .finish(),
            Self::Type {
                bounds,
                default,
                synthetic,
            } => f
                .debug_struct("Type")
                .field("bounds", bounds)
                .field("default", default)
                .field("synthetic", synthetic)
                .finish(),
            Self::Const { type_, default } => f
                .debug_struct("Const")
                .field("type_", type_)
                .field("default", default)
                .finish(),
        }
    }
}
impl crate::Debug for rustdoc_types::Generics {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Generics")
            .field("params", &self.params)
            .field("where_predicates", &self.where_predicates)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Header {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Header")
            .field("const_", &self.const_)
            .field("unsafe_", &self.unsafe_)
            .field("async_", &self.async_)
            .field("abi", &self.abi)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Id {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_tuple("Id").field(&self.0).finish()
    }
}
impl crate::Debug for rustdoc_types::Impl {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Impl")
            .field("is_unsafe", &self.is_unsafe)
            .field("generics", &self.generics)
            .field("provided_trait_methods", &self.provided_trait_methods)
            .field("trait_", &self.trait_)
            .field("for_", &self.for_)
            .field("items", &self.items)
            .field("negative", &self.negative)
            .field("synthetic", &self.synthetic)
            .field("blanket_impl", &self.blanket_impl)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Import {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Import")
            .field("source", &self.source)
            .field("name", &self.name)
            .field("id", &self.id)
            .field("glob", &self.glob)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Item {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Item")
            .field("id", &self.id)
            .field("crate_id", &self.crate_id)
            .field("name", &self.name)
            .field("span", &self.span)
            .field("visibility", &self.visibility)
            .field("docs", &self.docs)
            .field("links", &self.links)
            .field("attrs", &self.attrs)
            .field("deprecation", &self.deprecation)
            .field("inner", &self.inner)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::ItemEnum {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Module(__0) => {
                f.debug_tuple("Module").field(__0).finish();
            }
            Self::ExternCrate { name, rename } => f
                .debug_struct("ExternCrate")
                .field("name", name)
                .field("rename", rename)
                .finish(),
            Self::Import(__0) => {
                f.debug_tuple("Import").field(__0).finish();
            }
            Self::Union(__0) => {
                f.debug_tuple("Union").field(__0).finish();
            }
            Self::Struct(__0) => {
                f.debug_tuple("Struct").field(__0).finish();
            }
            Self::StructField(__0) => {
                f.debug_tuple("StructField").field(__0).finish();
            }
            Self::Enum(__0) => {
                f.debug_tuple("Enum").field(__0).finish();
            }
            Self::Variant(__0) => {
                f.debug_tuple("Variant").field(__0).finish();
            }
            Self::Function(__0) => {
                f.debug_tuple("Function").field(__0).finish();
            }
            Self::Trait(__0) => {
                f.debug_tuple("Trait").field(__0).finish();
            }
            Self::TraitAlias(__0) => {
                f.debug_tuple("TraitAlias").field(__0).finish();
            }
            Self::Method(__0) => {
                f.debug_tuple("Method").field(__0).finish();
            }
            Self::Impl(__0) => {
                f.debug_tuple("Impl").field(__0).finish();
            }
            Self::Typedef(__0) => {
                f.debug_tuple("Typedef").field(__0).finish();
            }
            Self::OpaqueTy(__0) => {
                f.debug_tuple("OpaqueTy").field(__0).finish();
            }
            Self::Constant(__0) => {
                f.debug_tuple("Constant").field(__0).finish();
            }
            Self::Static(__0) => {
                f.debug_tuple("Static").field(__0).finish();
            }
            Self::ForeignType => {
                f.debug_tuple("ForeignType").finish();
            }
            Self::Macro(__0) => {
                f.debug_tuple("Macro").field(__0).finish();
            }
            Self::ProcMacro(__0) => {
                f.debug_tuple("ProcMacro").field(__0).finish();
            }
            Self::PrimitiveType(__0) => {
                f.debug_tuple("PrimitiveType").field(__0).finish();
            }
            Self::AssocConst { type_, default } => f
                .debug_struct("AssocConst")
                .field("type_", type_)
                .field("default", default)
                .finish(),
            Self::AssocType {
                generics,
                bounds,
                default,
            } => f
                .debug_struct("AssocType")
                .field("generics", generics)
                .field("bounds", bounds)
                .field("default", default)
                .finish(),
        }
    }
}
impl crate::Debug for rustdoc_types::ItemKind {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Module => {
                f.debug_tuple("Module").finish();
            }
            Self::ExternCrate => {
                f.debug_tuple("ExternCrate").finish();
            }
            Self::Import => {
                f.debug_tuple("Import").finish();
            }
            Self::Struct => {
                f.debug_tuple("Struct").finish();
            }
            Self::StructField => {
                f.debug_tuple("StructField").finish();
            }
            Self::Union => {
                f.debug_tuple("Union").finish();
            }
            Self::Enum => {
                f.debug_tuple("Enum").finish();
            }
            Self::Variant => {
                f.debug_tuple("Variant").finish();
            }
            Self::Function => {
                f.debug_tuple("Function").finish();
            }
            Self::Typedef => {
                f.debug_tuple("Typedef").finish();
            }
            Self::OpaqueTy => {
                f.debug_tuple("OpaqueTy").finish();
            }
            Self::Constant => {
                f.debug_tuple("Constant").finish();
            }
            Self::Trait => {
                f.debug_tuple("Trait").finish();
            }
            Self::TraitAlias => {
                f.debug_tuple("TraitAlias").finish();
            }
            Self::Method => {
                f.debug_tuple("Method").finish();
            }
            Self::Impl => {
                f.debug_tuple("Impl").finish();
            }
            Self::Static => {
                f.debug_tuple("Static").finish();
            }
            Self::ForeignType => {
                f.debug_tuple("ForeignType").finish();
            }
            Self::Macro => {
                f.debug_tuple("Macro").finish();
            }
            Self::ProcAttribute => {
                f.debug_tuple("ProcAttribute").finish();
            }
            Self::ProcDerive => {
                f.debug_tuple("ProcDerive").finish();
            }
            Self::AssocConst => {
                f.debug_tuple("AssocConst").finish();
            }
            Self::AssocType => {
                f.debug_tuple("AssocType").finish();
            }
            Self::Primitive => {
                f.debug_tuple("Primitive").finish();
            }
            Self::Keyword => {
                f.debug_tuple("Keyword").finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::ItemSummary {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("ItemSummary")
            .field("crate_id", &self.crate_id)
            .field("path", &self.path)
            .field("kind", &self.kind)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::MacroKind {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Bang => {
                f.debug_tuple("Bang").finish();
            }
            Self::Attr => {
                f.debug_tuple("Attr").finish();
            }
            Self::Derive => {
                f.debug_tuple("Derive").finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::Method {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Method")
            .field("decl", &self.decl)
            .field("generics", &self.generics)
            .field("header", &self.header)
            .field("has_body", &self.has_body)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Module {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Module")
            .field("is_crate", &self.is_crate)
            .field("items", &self.items)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::OpaqueTy {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("OpaqueTy")
            .field("bounds", &self.bounds)
            .field("generics", &self.generics)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::ProcMacro {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("ProcMacro")
            .field("kind", &self.kind)
            .field("helpers", &self.helpers)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Span {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Span")
            .field("filename", &self.filename)
            .field("begin", &self.begin)
            .field("end", &self.end)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Static {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Static")
            .field("type_", &self.type_)
            .field("mutable", &self.mutable)
            .field("expr", &self.expr)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Struct {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Struct")
            .field("struct_type", &self.struct_type)
            .field("generics", &self.generics)
            .field("fields_stripped", &self.fields_stripped)
            .field("fields", &self.fields)
            .field("impls", &self.impls)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::StructType {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Plain => {
                f.debug_tuple("Plain").finish();
            }
            Self::Tuple => {
                f.debug_tuple("Tuple").finish();
            }
            Self::Unit => {
                f.debug_tuple("Unit").finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::Term {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Type(__0) => {
                f.debug_tuple("Type").field(__0).finish();
            }
            Self::Constant(__0) => {
                f.debug_tuple("Constant").field(__0).finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::Trait {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Trait")
            .field("is_auto", &self.is_auto)
            .field("is_unsafe", &self.is_unsafe)
            .field("items", &self.items)
            .field("generics", &self.generics)
            .field("bounds", &self.bounds)
            .field("implementations", &self.implementations)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::TraitAlias {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("TraitAlias")
            .field("generics", &self.generics)
            .field("params", &self.params)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::TraitBoundModifier {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::None => {
                f.debug_tuple("None").finish();
            }
            Self::Maybe => {
                f.debug_tuple("Maybe").finish();
            }
            Self::MaybeConst => {
                f.debug_tuple("MaybeConst").finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::Type {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::ResolvedPath {
                name,
                id,
                args,
                param_names,
            } => f
                .debug_struct("ResolvedPath")
                .field("name", name)
                .field("id", id)
                .field("args", args)
                .field("param_names", param_names)
                .finish(),
            Self::Generic(__0) => {
                f.debug_tuple("Generic").field(__0).finish();
            }
            Self::Primitive(__0) => {
                f.debug_tuple("Primitive").field(__0).finish();
            }
            Self::FunctionPointer(__0) => {
                f.debug_tuple("FunctionPointer").field(__0).finish();
            }
            Self::Tuple(__0) => {
                f.debug_tuple("Tuple").field(__0).finish();
            }
            Self::Slice(__0) => {
                f.debug_tuple("Slice").field(__0).finish();
            }
            Self::Array { type_, len } => f
                .debug_struct("Array")
                .field("type_", type_)
                .field("len", len)
                .finish(),
            Self::ImplTrait(__0) => {
                f.debug_tuple("ImplTrait").field(__0).finish();
            }
            Self::Infer => {
                f.debug_tuple("Infer").finish();
            }
            Self::RawPointer { mutable, type_ } => f
                .debug_struct("RawPointer")
                .field("mutable", mutable)
                .field("type_", type_)
                .finish(),
            Self::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => f
                .debug_struct("BorrowedRef")
                .field("lifetime", lifetime)
                .field("mutable", mutable)
                .field("type_", type_)
                .finish(),
            Self::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => f
                .debug_struct("QualifiedPath")
                .field("name", name)
                .field("args", args)
                .field("self_type", self_type)
                .field("trait_", trait_)
                .finish(),
        }
    }
}
impl crate::Debug for rustdoc_types::TypeBinding {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("TypeBinding")
            .field("name", &self.name)
            .field("args", &self.args)
            .field("binding", &self.binding)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::TypeBindingKind {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Equality(__0) => {
                f.debug_tuple("Equality").field(__0).finish();
            }
            Self::Constraint(__0) => {
                f.debug_tuple("Constraint").field(__0).finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::Typedef {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Typedef")
            .field("type_", &self.type_)
            .field("generics", &self.generics)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Union {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Union")
            .field("generics", &self.generics)
            .field("fields_stripped", &self.fields_stripped)
            .field("fields", &self.fields)
            .field("impls", &self.impls)
            .finish()
    }
}
impl crate::Debug for rustdoc_types::Variant {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Plain => {
                f.debug_tuple("Plain").finish();
            }
            Self::Tuple(__0) => {
                f.debug_tuple("Tuple").field(__0).finish();
            }
            Self::Struct(__0) => {
                f.debug_tuple("Struct").field(__0).finish();
            }
        }
    }
}
impl crate::Debug for rustdoc_types::Visibility {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::Public => {
                f.debug_tuple("Public").finish();
            }
            Self::Default => {
                f.debug_tuple("Default").finish();
            }
            Self::Crate => {
                f.debug_tuple("Crate").finish();
            }
            Self::Restricted { parent, path } => f
                .debug_struct("Restricted")
                .field("parent", parent)
                .field("path", path)
                .finish(),
        }
    }
}
impl crate::Debug for rustdoc_types::WherePredicate {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            Self::BoundPredicate {
                type_,
                bounds,
                generic_params,
            } => f
                .debug_struct("BoundPredicate")
                .field("type_", type_)
                .field("bounds", bounds)
                .field("generic_params", generic_params)
                .finish(),
            Self::RegionPredicate { lifetime, bounds } => f
                .debug_struct("RegionPredicate")
                .field("lifetime", lifetime)
                .field("bounds", bounds)
                .finish(),
            Self::EqPredicate { lhs, rhs } => f
                .debug_struct("EqPredicate")
                .field("lhs", lhs)
                .field("rhs", rhs)
                .finish(),
        }
    }
}
