use anyhow::{anyhow, bail, Result};
use rustdoc_types::{
    Crate, Enum, GenericParamDefKind, Generics, Id, Item, ItemEnum, Struct, Variant,
};
use std::fmt::Write;

pub(crate) fn generate(krate: &Crate) -> Result<String> {
    let mut out = String::new();
    out.push_str("// AUTOGENERATED FILE, DO NOT EDIT\n");
    out.push_str("//\n");

    let name = krate.index[&krate.root]
        .name
        .as_ref()
        .ok_or_else(|| anyhow!("No name"))?;
    let version = krate
        .crate_version
        .as_ref()
        .ok_or_else(|| anyhow!("No version"))?;

    let root_id = krate.index[&krate.root].crate_id;

    writeln!(out, "// Crate Name: `{}`", name)?;
    writeln!(out, "// Crate Version: `{}`", version)?;
    writeln!(out, "")?;

    // writeln!(out, "#![rustfmt::skip]")?;

    for (id, item) in &krate.index {
        if item.crate_id != root_id {
            continue;
        }

        match &item.inner {
            ItemEnum::Enum(enumm) => {
                generate_enum(&mut out, id, item, enumm, &krate)?;
            }
            ItemEnum::Struct(strukt) => {
                generate_struct(&mut out, id, item, strukt, &krate)?;
            }
            _ => {}
        }
    }

    Ok(out)
}

fn generate_struct(
    out: &mut String,
    id: &Id,
    item: &Item,
    strukt: &Struct,
    krate: &Crate,
) -> Result<()> {
    let path = krate.paths[id].path.join("::");

    let (igen, tgen, where_) = extract_generics(&strukt.generics)?;

    writeln!(
        out,
        "impl<{igen}> crate::Debug for {path}<{tgen}>  where {where_} {{"
    )?;
    writeln!(out, "    fn fmt(&self, f: &mut crate::Formatter) {{")?;

    let name = item.name.as_ref().ok_or_else(|| anyhow!("No name"))?;

    let striped = strukt.fields_stripped;

    if striped {
        writeln!(out, "        // Warning: Striped Fields")?;
    }

    match strukt.struct_type {
        rustdoc_types::StructType::Plain => {
            writeln!(out, "        f.debug_struct({name:?})")?;
            for i in &strukt.fields {
                let f_name = krate.index[i]
                    .name
                    .as_ref()
                    .ok_or_else(|| anyhow!("No name"))?;
                writeln!(out, "            .field({f_name:?}, &self.{f_name})")?;
            }
            writeln!(out, "            .finish()")?;
        }
        rustdoc_types::StructType::Tuple => {}
        rustdoc_types::StructType::Unit => {
            writeln!(out, "        f.debug_struct({name:?})")?;
            writeln!(out, "            .finish()")?;
        }
    }

    writeln!(out, "    }}\n}}")?;

    Ok(())
}

fn generate_enum(
    out: &mut String,
    id: &Id,
    item: &Item,
    enumm: &Enum,
    krate: &Crate,
) -> Result<()> {
    let path = krate.paths[id].path.join("::");

    let (igen, tgen, where_) = extract_generics(&enumm.generics)?;

    writeln!(
        out,
        "impl<{igen}>  crate::Debug for {path}<{tgen}> where {where_} {{"
    )?;
    writeln!(out, "    fn fmt(&self, f: &mut crate::Formatter) {{")?;
    writeln!(out, "        match self {{")?;

    for i in &enumm.variants {
        let v_item = &krate.index[i];
        let v_name = v_item.name.as_ref().ok_or_else(|| anyhow!("No name"))?;

        if let ItemEnum::Variant(varient) = &v_item.inner {
            write!(out, "            {path}::{v_name} ")?;
            match varient {
                Variant::Plain => writeln!(out, "=> {{ f.debug_tuple({v_name:?}).finish(); }}")?,
                Variant::Tuple(ids) => {
                    write!(out, "(")?;
                    for i in 0..ids.len() {
                        write!(out, "__{}, ", i)?;
                    }
                    write!(out, ") => {{ f.debug_tuple({v_name:?})")?;
                    for i in 0..ids.len() {
                        write!(out, ".field(__{i})")?;
                    }
                    writeln!(out, ".finish(); }}")?;
                }
                Variant::Struct(ids) => {
                    write!(out, "{{")?;

                    for i in ids {
                        let f_name = krate.index[i]
                            .name
                            .as_ref()
                            .ok_or_else(|| anyhow!("No name"))?;
                        write!(out, "{f_name}, ")?;
                    }

                    writeln!(out, " }} => {{")?;
                    writeln!(out, "            f.debug_struct({v_name:?})")?;
                    for i in ids {
                        let f_name = krate.index[i]
                            .name
                            .as_ref()
                            .ok_or_else(|| anyhow!("No name"))?;
                        writeln!(out, "                .field({f_name:?}, {f_name})")?;
                    }
                    writeln!(out, "                .finish()")?;
                    writeln!(out, "        }}")?;
                }
            }
        } else {
            bail!("Excpeced Variant");
        }
    }

    // TODO: Document #[non_exaustive] in JSON
    if enumm.variants_stripped || item.attrs.iter().any(|x| x == "#[non_exhaustive]") {
        writeln!(out, "            _ => {{ \"???\".fmt(f) }}")?;
    }

    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    Ok(())
}

fn extract_generics(generics: &Generics) -> Result<(String, String, String)> {
    let mut impl_ = String::new();
    let mut type_ = String::new();
    let mut where_ = String::new();

    // impl {impl_} debug3 for type {type_}

    for i in &generics.params {
        match &i.kind {
            GenericParamDefKind::Lifetime { .. } => {
                write!(impl_, "{},", i.name)?;
                write!(type_, "{},", i.name)?;
            }
            GenericParamDefKind::Type { .. } => {
                write!(impl_, "{},", i.name)?;
                write!(type_, "{},", i.name)?;
                write!(where_, "{} : crate::Debug,", i.name)?;
            }
            GenericParamDefKind::Const { .. } => todo!(),
        }
    }
    // Remove trailing comma
    impl_.pop();
    type_.pop();
    where_.pop();

    Ok((impl_, type_, where_))
}
