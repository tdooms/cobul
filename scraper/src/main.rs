use convert_case::{Case, Casing};
use std::collections::HashMap;
use std::io::{Error, Write};

#[derive(Clone, Debug, serde::Deserialize)]
struct Icon {
    free: Vec<String>,
}

fn export_kind(out: &mut impl Write, icons: &[(String, Icon)], kind: &String) -> Result<(), Error> {
    let ident = kind.to_case(Case::Pascal);
    out.write(b"#[derive(Clone, Copy, derive_more::Display)]\n")?;
    out.write_fmt(format_args!("pub enum {} {{\n", ident))?;

    for (name, _) in icons.iter().filter(|(_, icon)| icon.free.contains(kind)) {
        let camel = name.to_case(Case::Pascal);
        out.write_fmt(format_args!(
            "\t#[display(fmt = \"fa-{} fa-{}\")]\n",
            kind, name
        ))?;
        out.write_fmt(format_args!("\t{},\n", camel))?
    }

    out.write(b"}\n\n")?;

    out.write_fmt(format_args!(
        "impl yew::html::IntoPropValue<String> for {} {{\n",
        ident
    ))?;
    out.write(b"\tfn into_prop_value(self) -> String { self.to_string() }\n")?;
    out.write(b"}\n\n")?;

    out.write_fmt(format_args!(
        "impl yew::html::IntoPropValue<Option<String>> for {} {{\n",
        ident
    ))?;
    out.write(b"\tfn into_prop_value(self) -> Option<String> { Some(self.to_string()) }\n")?;
    out.write(b"}\n\n")?;

    out.write_fmt(format_args!("impl Icon for {} {{ }}\n\n", ident))?;
    Ok(())
}

fn export_all(out: &mut impl Write, icons: &[(String, Icon)]) -> Result<(), Error> {
    out.write(b"pub trait Icon: yew::html::IntoPropValue<String> {} \n\n")?;

    for kind in ["solid", "brands", "regular"].map(ToString::to_string) {
        export_kind(out, icons, &kind)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::File::open("scraper/icons.json")?;
    let mut output = std::fs::File::create("scraper/output.rs")?;

    let icons: HashMap<String, Icon> = serde_json::from_reader(input)?;
    let mut icons: Vec<(String, Icon)> = icons.into_iter().collect();
    icons.sort_by(|(a, _), (b, _)| a.cmp(&b));

    export_all(&mut output, &icons)?;

    Ok(())
}
