// AUTOGENERATED FILE, DO NOT EDIT
//
// Crate Name: `rustdoc_types`
// Crate Version: `0.10.0`

impl<> crate::Debug for rustdoc_types::GenericParamDef<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("GenericParamDef")
            .field("name", &self.name)
            .field("kind", &self.kind)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Typedef<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Typedef")
            .field("type_", &self.type_)
            .field("generics", &self.generics)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Id<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
    }
}
impl<> crate::Debug for rustdoc_types::Method<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Method")
            .field("decl", &self.decl)
            .field("generics", &self.generics)
            .field("header", &self.header)
            .field("has_body", &self.has_body)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Union<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Union")
            .field("generics", &self.generics)
            .field("fields_stripped", &self.fields_stripped)
            .field("fields", &self.fields)
            .field("impls", &self.impls)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Header<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Header")
            .field("const_", &self.const_)
            .field("unsafe_", &self.unsafe_)
            .field("async_", &self.async_)
            .field("abi", &self.abi)
            .finish()
    }
}
impl<>  crate::Debug for rustdoc_types::GenericArg<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::GenericArg::Lifetime (__0, ) => { f.debug_tuple("Lifetime").field(__0).finish(); }
            rustdoc_types::GenericArg::Type (__0, ) => { f.debug_tuple("Type").field(__0).finish(); }
            rustdoc_types::GenericArg::Const (__0, ) => { f.debug_tuple("Const").field(__0).finish(); }
            rustdoc_types::GenericArg::Infer => { f.debug_tuple("Infer").finish(); }
        }
    }
}
impl<>  crate::Debug for rustdoc_types::TraitBoundModifier<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::TraitBoundModifier::None => { f.debug_tuple("None").finish(); }
            rustdoc_types::TraitBoundModifier::Maybe => { f.debug_tuple("Maybe").finish(); }
            rustdoc_types::TraitBoundModifier::MaybeConst => { f.debug_tuple("MaybeConst").finish(); }
        }
    }
}
impl<>  crate::Debug for rustdoc_types::MacroKind<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::MacroKind::Bang => { f.debug_tuple("Bang").finish(); }
            rustdoc_types::MacroKind::Attr => { f.debug_tuple("Attr").finish(); }
            rustdoc_types::MacroKind::Derive => { f.debug_tuple("Derive").finish(); }
        }
    }
}
impl<> crate::Debug for rustdoc_types::Deprecation<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Deprecation")
            .field("since", &self.since)
            .field("note", &self.note)
            .finish()
    }
}
impl<>  crate::Debug for rustdoc_types::Term<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::Term::Type (__0, ) => { f.debug_tuple("Type").field(__0).finish(); }
            rustdoc_types::Term::Constant (__0, ) => { f.debug_tuple("Constant").field(__0).finish(); }
        }
    }
}
impl<>  crate::Debug for rustdoc_types::StructType<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::StructType::Plain => { f.debug_tuple("Plain").finish(); }
            rustdoc_types::StructType::Tuple => { f.debug_tuple("Tuple").finish(); }
            rustdoc_types::StructType::Unit => { f.debug_tuple("Unit").finish(); }
        }
    }
}
impl<>  crate::Debug for rustdoc_types::WherePredicate<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::WherePredicate::BoundPredicate {type_, bounds,  } => {
            f.debug_struct("BoundPredicate")
                .field("type_", type_)
                .field("bounds", bounds)
                .finish()
        }
            rustdoc_types::WherePredicate::RegionPredicate {lifetime, bounds,  } => {
            f.debug_struct("RegionPredicate")
                .field("lifetime", lifetime)
                .field("bounds", bounds)
                .finish()
        }
            rustdoc_types::WherePredicate::EqPredicate {lhs, rhs,  } => {
            f.debug_struct("EqPredicate")
                .field("lhs", lhs)
                .field("rhs", rhs)
                .finish()
        }
        }
    }
}
impl<>  crate::Debug for rustdoc_types::Type<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::Type::ResolvedPath {name, id, args, param_names,  } => {
            f.debug_struct("ResolvedPath")
                .field("name", name)
                .field("id", id)
                .field("args", args)
                .field("param_names", param_names)
                .finish()
        }
            rustdoc_types::Type::Generic (__0, ) => { f.debug_tuple("Generic").field(__0).finish(); }
            rustdoc_types::Type::Primitive (__0, ) => { f.debug_tuple("Primitive").field(__0).finish(); }
            rustdoc_types::Type::FunctionPointer (__0, ) => { f.debug_tuple("FunctionPointer").field(__0).finish(); }
            rustdoc_types::Type::Tuple (__0, ) => { f.debug_tuple("Tuple").field(__0).finish(); }
            rustdoc_types::Type::Slice (__0, ) => { f.debug_tuple("Slice").field(__0).finish(); }
            rustdoc_types::Type::Array {type_, len,  } => {
            f.debug_struct("Array")
                .field("type_", type_)
                .field("len", len)
                .finish()
        }
            rustdoc_types::Type::ImplTrait (__0, ) => { f.debug_tuple("ImplTrait").field(__0).finish(); }
            rustdoc_types::Type::Infer => { f.debug_tuple("Infer").finish(); }
            rustdoc_types::Type::RawPointer {mutable, type_,  } => {
            f.debug_struct("RawPointer")
                .field("mutable", mutable)
                .field("type_", type_)
                .finish()
        }
            rustdoc_types::Type::BorrowedRef {lifetime, mutable, type_,  } => {
            f.debug_struct("BorrowedRef")
                .field("lifetime", lifetime)
                .field("mutable", mutable)
                .field("type_", type_)
                .finish()
        }
            rustdoc_types::Type::QualifiedPath {name, args, self_type, trait_,  } => {
            f.debug_struct("QualifiedPath")
                .field("name", name)
                .field("args", args)
                .field("self_type", self_type)
                .field("trait_", trait_)
                .finish()
        }
        }
    }
}
impl<>  crate::Debug for rustdoc_types::GenericBound<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::GenericBound::TraitBound {trait_, generic_params, modifier,  } => {
            f.debug_struct("TraitBound")
                .field("trait_", trait_)
                .field("generic_params", generic_params)
                .field("modifier", modifier)
                .finish()
        }
            rustdoc_types::GenericBound::Outlives (__0, ) => { f.debug_tuple("Outlives").field(__0).finish(); }
        }
    }
}
impl<> crate::Debug for rustdoc_types::Enum<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Enum")
            .field("generics", &self.generics)
            .field("variants_stripped", &self.variants_stripped)
            .field("variants", &self.variants)
            .field("impls", &self.impls)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Item<>  where  {
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
impl<> crate::Debug for rustdoc_types::TraitAlias<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("TraitAlias")
            .field("generics", &self.generics)
            .field("params", &self.params)
            .finish()
    }
}
impl<>  crate::Debug for rustdoc_types::TypeBindingKind<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::TypeBindingKind::Equality (__0, ) => { f.debug_tuple("Equality").field(__0).finish(); }
            rustdoc_types::TypeBindingKind::Constraint (__0, ) => { f.debug_tuple("Constraint").field(__0).finish(); }
        }
    }
}
impl<> crate::Debug for rustdoc_types::FunctionPointer<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("FunctionPointer")
            .field("decl", &self.decl)
            .field("generic_params", &self.generic_params)
            .field("header", &self.header)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::ProcMacro<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("ProcMacro")
            .field("kind", &self.kind)
            .field("helpers", &self.helpers)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Span<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Span")
            .field("filename", &self.filename)
            .field("begin", &self.begin)
            .field("end", &self.end)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::TypeBinding<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("TypeBinding")
            .field("name", &self.name)
            .field("args", &self.args)
            .field("binding", &self.binding)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Constant<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Constant")
            .field("type_", &self.type_)
            .field("expr", &self.expr)
            .field("value", &self.value)
            .field("is_literal", &self.is_literal)
            .finish()
    }
}
impl<>  crate::Debug for rustdoc_types::GenericParamDefKind<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::GenericParamDefKind::Lifetime {outlives,  } => {
            f.debug_struct("Lifetime")
                .field("outlives", outlives)
                .finish()
        }
            rustdoc_types::GenericParamDefKind::Type {bounds, default, synthetic,  } => {
            f.debug_struct("Type")
                .field("bounds", bounds)
                .field("default", default)
                .field("synthetic", synthetic)
                .finish()
        }
            rustdoc_types::GenericParamDefKind::Const {type_, default,  } => {
            f.debug_struct("Const")
                .field("type_", type_)
                .field("default", default)
                .finish()
        }
        }
    }
}
impl<>  crate::Debug for rustdoc_types::ItemEnum<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::ItemEnum::Module (__0, ) => { f.debug_tuple("Module").field(__0).finish(); }
            rustdoc_types::ItemEnum::ExternCrate {name, rename,  } => {
            f.debug_struct("ExternCrate")
                .field("name", name)
                .field("rename", rename)
                .finish()
        }
            rustdoc_types::ItemEnum::Import (__0, ) => { f.debug_tuple("Import").field(__0).finish(); }
            rustdoc_types::ItemEnum::Union (__0, ) => { f.debug_tuple("Union").field(__0).finish(); }
            rustdoc_types::ItemEnum::Struct (__0, ) => { f.debug_tuple("Struct").field(__0).finish(); }
            rustdoc_types::ItemEnum::StructField (__0, ) => { f.debug_tuple("StructField").field(__0).finish(); }
            rustdoc_types::ItemEnum::Enum (__0, ) => { f.debug_tuple("Enum").field(__0).finish(); }
            rustdoc_types::ItemEnum::Variant (__0, ) => { f.debug_tuple("Variant").field(__0).finish(); }
            rustdoc_types::ItemEnum::Function (__0, ) => { f.debug_tuple("Function").field(__0).finish(); }
            rustdoc_types::ItemEnum::Trait (__0, ) => { f.debug_tuple("Trait").field(__0).finish(); }
            rustdoc_types::ItemEnum::TraitAlias (__0, ) => { f.debug_tuple("TraitAlias").field(__0).finish(); }
            rustdoc_types::ItemEnum::Method (__0, ) => { f.debug_tuple("Method").field(__0).finish(); }
            rustdoc_types::ItemEnum::Impl (__0, ) => { f.debug_tuple("Impl").field(__0).finish(); }
            rustdoc_types::ItemEnum::Typedef (__0, ) => { f.debug_tuple("Typedef").field(__0).finish(); }
            rustdoc_types::ItemEnum::OpaqueTy (__0, ) => { f.debug_tuple("OpaqueTy").field(__0).finish(); }
            rustdoc_types::ItemEnum::Constant (__0, ) => { f.debug_tuple("Constant").field(__0).finish(); }
            rustdoc_types::ItemEnum::Static (__0, ) => { f.debug_tuple("Static").field(__0).finish(); }
            rustdoc_types::ItemEnum::ForeignType => { f.debug_tuple("ForeignType").finish(); }
            rustdoc_types::ItemEnum::Macro (__0, ) => { f.debug_tuple("Macro").field(__0).finish(); }
            rustdoc_types::ItemEnum::ProcMacro (__0, ) => { f.debug_tuple("ProcMacro").field(__0).finish(); }
            rustdoc_types::ItemEnum::PrimitiveType (__0, ) => { f.debug_tuple("PrimitiveType").field(__0).finish(); }
            rustdoc_types::ItemEnum::AssocConst {type_, default,  } => {
            f.debug_struct("AssocConst")
                .field("type_", type_)
                .field("default", default)
                .finish()
        }
            rustdoc_types::ItemEnum::AssocType {generics, bounds, default,  } => {
            f.debug_struct("AssocType")
                .field("generics", generics)
                .field("bounds", bounds)
                .field("default", default)
                .finish()
        }
        }
    }
}
impl<> crate::Debug for rustdoc_types::Generics<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Generics")
            .field("params", &self.params)
            .field("where_predicates", &self.where_predicates)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Import<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Import")
            .field("source", &self.source)
            .field("name", &self.name)
            .field("id", &self.id)
            .field("glob", &self.glob)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Trait<>  where  {
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
impl<> crate::Debug for rustdoc_types::ItemSummary<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("ItemSummary")
            .field("crate_id", &self.crate_id)
            .field("path", &self.path)
            .field("kind", &self.kind)
            .finish()
    }
}
impl<>  crate::Debug for rustdoc_types::Variant<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::Variant::Plain => { f.debug_tuple("Plain").finish(); }
            rustdoc_types::Variant::Tuple (__0, ) => { f.debug_tuple("Tuple").field(__0).finish(); }
            rustdoc_types::Variant::Struct (__0, ) => { f.debug_tuple("Struct").field(__0).finish(); }
        }
    }
}
impl<> crate::Debug for rustdoc_types::Impl<>  where  {
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
impl<>  crate::Debug for rustdoc_types::ItemKind<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::ItemKind::Module => { f.debug_tuple("Module").finish(); }
            rustdoc_types::ItemKind::ExternCrate => { f.debug_tuple("ExternCrate").finish(); }
            rustdoc_types::ItemKind::Import => { f.debug_tuple("Import").finish(); }
            rustdoc_types::ItemKind::Struct => { f.debug_tuple("Struct").finish(); }
            rustdoc_types::ItemKind::StructField => { f.debug_tuple("StructField").finish(); }
            rustdoc_types::ItemKind::Union => { f.debug_tuple("Union").finish(); }
            rustdoc_types::ItemKind::Enum => { f.debug_tuple("Enum").finish(); }
            rustdoc_types::ItemKind::Variant => { f.debug_tuple("Variant").finish(); }
            rustdoc_types::ItemKind::Function => { f.debug_tuple("Function").finish(); }
            rustdoc_types::ItemKind::Typedef => { f.debug_tuple("Typedef").finish(); }
            rustdoc_types::ItemKind::OpaqueTy => { f.debug_tuple("OpaqueTy").finish(); }
            rustdoc_types::ItemKind::Constant => { f.debug_tuple("Constant").finish(); }
            rustdoc_types::ItemKind::Trait => { f.debug_tuple("Trait").finish(); }
            rustdoc_types::ItemKind::TraitAlias => { f.debug_tuple("TraitAlias").finish(); }
            rustdoc_types::ItemKind::Method => { f.debug_tuple("Method").finish(); }
            rustdoc_types::ItemKind::Impl => { f.debug_tuple("Impl").finish(); }
            rustdoc_types::ItemKind::Static => { f.debug_tuple("Static").finish(); }
            rustdoc_types::ItemKind::ForeignType => { f.debug_tuple("ForeignType").finish(); }
            rustdoc_types::ItemKind::Macro => { f.debug_tuple("Macro").finish(); }
            rustdoc_types::ItemKind::ProcAttribute => { f.debug_tuple("ProcAttribute").finish(); }
            rustdoc_types::ItemKind::ProcDerive => { f.debug_tuple("ProcDerive").finish(); }
            rustdoc_types::ItemKind::AssocConst => { f.debug_tuple("AssocConst").finish(); }
            rustdoc_types::ItemKind::AssocType => { f.debug_tuple("AssocType").finish(); }
            rustdoc_types::ItemKind::Primitive => { f.debug_tuple("Primitive").finish(); }
            rustdoc_types::ItemKind::Keyword => { f.debug_tuple("Keyword").finish(); }
        }
    }
}
impl<>  crate::Debug for rustdoc_types::Visibility<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::Visibility::Public => { f.debug_tuple("Public").finish(); }
            rustdoc_types::Visibility::Default => { f.debug_tuple("Default").finish(); }
            rustdoc_types::Visibility::Crate => { f.debug_tuple("Crate").finish(); }
            rustdoc_types::Visibility::Restricted {parent, path,  } => {
            f.debug_struct("Restricted")
                .field("parent", parent)
                .field("path", path)
                .finish()
        }
        }
    }
}
impl<>  crate::Debug for rustdoc_types::Abi<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::Abi::Rust => { f.debug_tuple("Rust").finish(); }
            rustdoc_types::Abi::C {unwind,  } => {
            f.debug_struct("C")
                .field("unwind", unwind)
                .finish()
        }
            rustdoc_types::Abi::Cdecl {unwind,  } => {
            f.debug_struct("Cdecl")
                .field("unwind", unwind)
                .finish()
        }
            rustdoc_types::Abi::Stdcall {unwind,  } => {
            f.debug_struct("Stdcall")
                .field("unwind", unwind)
                .finish()
        }
            rustdoc_types::Abi::Fastcall {unwind,  } => {
            f.debug_struct("Fastcall")
                .field("unwind", unwind)
                .finish()
        }
            rustdoc_types::Abi::Aapcs {unwind,  } => {
            f.debug_struct("Aapcs")
                .field("unwind", unwind)
                .finish()
        }
            rustdoc_types::Abi::Win64 {unwind,  } => {
            f.debug_struct("Win64")
                .field("unwind", unwind)
                .finish()
        }
            rustdoc_types::Abi::SysV64 {unwind,  } => {
            f.debug_struct("SysV64")
                .field("unwind", unwind)
                .finish()
        }
            rustdoc_types::Abi::System {unwind,  } => {
            f.debug_struct("System")
                .field("unwind", unwind)
                .finish()
        }
            rustdoc_types::Abi::Other (__0, ) => { f.debug_tuple("Other").field(__0).finish(); }
        }
    }
}
impl<> crate::Debug for rustdoc_types::Module<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Module")
            .field("is_crate", &self.is_crate)
            .field("items", &self.items)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::OpaqueTy<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("OpaqueTy")
            .field("bounds", &self.bounds)
            .field("generics", &self.generics)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Function<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Function")
            .field("decl", &self.decl)
            .field("generics", &self.generics)
            .field("header", &self.header)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::FnDecl<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("FnDecl")
            .field("inputs", &self.inputs)
            .field("output", &self.output)
            .field("c_variadic", &self.c_variadic)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Crate<>  where  {
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
impl<> crate::Debug for rustdoc_types::ExternalCrate<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("ExternalCrate")
            .field("name", &self.name)
            .field("html_root_url", &self.html_root_url)
            .finish()
    }
}
impl<> crate::Debug for rustdoc_types::Struct<>  where  {
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
impl<>  crate::Debug for rustdoc_types::GenericArgs<> where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        match self {
            rustdoc_types::GenericArgs::AngleBracketed {args, bindings,  } => {
            f.debug_struct("AngleBracketed")
                .field("args", args)
                .field("bindings", bindings)
                .finish()
        }
            rustdoc_types::GenericArgs::Parenthesized {inputs, output,  } => {
            f.debug_struct("Parenthesized")
                .field("inputs", inputs)
                .field("output", output)
                .finish()
        }
        }
    }
}
impl<> crate::Debug for rustdoc_types::Static<>  where  {
    fn fmt(&self, f: &mut crate::Formatter) {
        f.debug_struct("Static")
            .field("type_", &self.type_)
            .field("mutable", &self.mutable)
            .field("expr", &self.expr)
            .finish()
    }
}
