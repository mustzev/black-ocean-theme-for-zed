//! Generates `themes/black-ocean.json` from the Black Ocean palettes.
//!
//! Colors are taken from the original VS Code theme by Alex Oxthorn
//! (https://github.com/Zamerick/black-ocean).
//!
//! Run from the repo root: `cargo run --manifest-path generator/Cargo.toml`

use serde_json::{json, Map, Value};
use std::path::Path;

/// Colors shared by both VS Code variants.
struct Base {
    bg_darkest: &'static str, // titleBar
    bg_dark: &'static str,    // activityBar, panel, terminal
    bg: &'static str,         // editor, sidebar
    bg_light: &'static str,   // line highlight, tabs, notifications
    bg_lighter: &'static str, // inactive list selection
    line: &'static str,       // borders, whitespace, line numbers
    comment: &'static str,
    muted: &'static str,
    fg: &'static str,
    red: &'static str,
    orange: &'static str,
    yellow: &'static str,
    magenta: &'static str,
}

const BASE: Base = Base {
    bg_darkest: "#07090a",
    bg_dark: "#0c0e10",
    bg: "#101316",
    bg_light: "#1b2025",
    bg_lighter: "#242b31",
    line: "#3b4651",
    comment: "#60778c",
    muted: "#8a9daf",
    fg: "#dfdfdf",
    red: "#e61f44",
    orange: "#f7b83d",
    yellow: "#edce72",
    magenta: "#9100ae",
};

/// Colors that differ between Black Ocean and Black Ocean Neon.
struct Variant {
    name: &'static str,
    string: &'static str,
    teal: &'static str,  // numbers, functions
    green: &'static str, // constants, types, attributes
    blue: &'static str,  // keywords, tags
    cyan: &'static str,
    lime: &'static str,
    bright_green: &'static str,
}

const VARIANTS: [Variant; 2] = [
    Variant {
        name: "Black Ocean",
        string: "#7ebea0",
        teal: "#15b8ae",
        green: "#019d76",
        blue: "#007aae",
        cyan: "#0490a0",
        lime: "#a7da1e",
        bright_green: "#00c200",
    },
    Variant {
        name: "Black Ocean Neon",
        string: "#44e8b0",
        teal: "#00f0dd",
        green: "#00df88",
        blue: "#00b8ff",
        cyan: "#00c8f0",
        lime: "#b8ff00",
        bright_green: "#00ff9f",
    },
];

/// Appends a hex alpha channel to a `#rrggbb` color.
fn a(color: &str, alpha: u8) -> String {
    format!("{color}{alpha:02x}")
}

fn rgb(color: &str) -> [f64; 3] {
    let channel = |i: usize| u8::from_str_radix(&color[i..i + 2], 16).unwrap() as f64;
    [channel(1), channel(3), channel(5)]
}

/// Opaque blend of `fg` over `bg` with opacity `t`.
fn mix(fg: &str, bg: &str, t: f64) -> String {
    let (f, b) = (rgb(fg), rgb(bg));
    let c: Vec<String> = (0..3)
        .map(|i| format!("{:02x}", (f[i] * t + b[i] * (1.0 - t)).round() as u8))
        .collect();
    format!("#{}", c.concat())
}

fn s(color: impl Into<String>) -> Value {
    json!({ "color": color.into(), "font_style": null, "font_weight": null })
}

fn italic(color: impl Into<String>) -> Value {
    json!({ "color": color.into(), "font_style": "italic", "font_weight": null })
}

fn bold(color: impl Into<String>) -> Value {
    json!({ "color": color.into(), "font_style": null, "font_weight": 700 })
}

/// Builds a JSON object from key/value pairs, keeping insertion order.
fn object<K: Into<String>>(pairs: Vec<(K, Value)>) -> Map<String, Value> {
    pairs.into_iter().map(|(k, v)| (k.into(), v)).collect()
}

fn theme(v: &Variant) -> Value {
    let c = &BASE;
    // The VS Code theme uses #007aae8e for its blue chrome; flatten it so
    // Zed's layered surfaces don't compound the alpha.
    let chrome = mix(v.blue, c.bg_dark, 0x8e as f64 / 255.0);
    let selection = a(v.blue, 0x8e);
    let dim = |color: &str| mix(color, c.bg_dark, 0.7);

    let mut style = object(vec![
        // Borders
        ("border", json!(c.bg_lighter)),
        ("border.variant", json!(c.bg_light)),
        ("border.focused", json!(a(v.blue, 0xcc))),
        ("border.selected", json!(selection)),
        ("border.transparent", json!("#00000000")),
        ("border.disabled", json!(c.bg_light)),
        // Surfaces
        ("background", json!(c.bg_dark)),
        ("surface.background", json!(c.bg_dark)),
        ("elevated_surface.background", json!(c.bg_light)),
        ("drop_target.background", json!(a(v.blue, 0x40))),
        // Elements
        ("element.background", json!(c.bg_light)),
        ("element.hover", json!(c.bg_lighter)),
        ("element.active", json!(c.line)),
        ("element.selected", json!(a(v.blue, 0x55))),
        ("element.disabled", json!(c.bg_light)),
        ("ghost_element.background", json!("#00000000")),
        ("ghost_element.hover", json!(c.bg_light)),
        ("ghost_element.active", json!(c.bg_lighter)),
        ("ghost_element.selected", json!(a(v.blue, 0x40))),
        ("ghost_element.disabled", json!("#00000000")),
        // Text & icons
        ("text", json!(c.fg)),
        ("text.muted", json!(c.muted)),
        ("text.placeholder", json!(c.comment)),
        ("text.disabled", json!(c.line)),
        ("text.accent", json!(v.teal)),
        ("icon", json!(c.fg)),
        ("icon.muted", json!(c.muted)),
        ("icon.placeholder", json!(c.comment)),
        ("icon.disabled", json!(c.line)),
        ("icon.accent", json!(v.teal)),
        ("link_text.hover", json!(v.teal)),
        // Chrome
        ("title_bar.background", json!(c.bg_darkest)),
        ("title_bar.inactive_background", json!(c.bg_darkest)),
        ("status_bar.background", json!(chrome)),
        ("toolbar.background", json!(c.bg)),
        ("tab_bar.background", json!(c.bg_light)),
        ("tab.inactive_background", json!(c.bg_light)),
        ("tab.active_background", json!(c.bg)),
        ("panel.background", json!(c.bg)),
        ("panel.focused_border", json!(a(v.blue, 0xcc))),
        ("panel.indent_guide", json!(c.bg_lighter)),
        ("panel.indent_guide_active", json!(c.line)),
        ("panel.indent_guide_hover", json!(c.comment)),
        ("pane.focused_border", Value::Null),
        ("pane_group.border", json!(c.line)),
        ("search.match_background", json!(a(c.muted, 0x55))),
        ("search.active_match_background", json!(a(c.orange, 0x66))),
        // Scrollbar
        ("scrollbar.thumb.background", json!(a(c.line, 0x99))),
        ("scrollbar.thumb.hover_background", json!(selection)),
        ("scrollbar.thumb.border", json!("#00000000")),
        ("scrollbar.track.background", json!("#00000000")),
        ("scrollbar.track.border", json!("#00000000")),
        // Editor
        ("editor.foreground", json!(c.fg)),
        ("editor.background", json!(c.bg)),
        ("editor.gutter.background", json!(c.bg)),
        ("editor.subheader.background", json!(c.bg_dark)),
        ("editor.active_line.background", json!(c.bg_light)),
        ("editor.highlighted_line.background", json!(c.bg_light)),
        ("editor.line_number", json!(c.line)),
        ("editor.active_line_number", json!(c.fg)),
        ("editor.hover_line_number", json!(c.muted)),
        ("editor.invisible", json!(c.line)),
        ("editor.wrap_guide", json!(a(c.line, 0x55))),
        ("editor.active_wrap_guide", json!(a(c.line, 0xaa))),
        ("editor.indent_guide", json!(c.bg_lighter)),
        ("editor.indent_guide_active", json!(c.line)),
        ("editor.document_highlight.read_background", json!("#d2e0ff1f")),
        ("editor.document_highlight.write_background", json!(a(v.blue, 0x4d))),
        ("editor.document_highlight.bracket_background", json!(a(v.blue, 0x55))),
        // Terminal. The original's bright yellow/blue were plain #dfdfdf and
        // bright magenta was yellow; those are replaced with distinct hues.
        ("terminal.background", json!(c.bg_dark)),
        ("terminal.foreground", json!(c.fg)),
        ("terminal.bright_foreground", json!("#ffffff")),
        ("terminal.dim_foreground", json!(c.muted)),
        ("terminal.ansi.black", json!(c.bg_light)),
        ("terminal.ansi.bright_black", json!(c.line)),
        ("terminal.ansi.dim_black", json!(c.bg_dark)),
        ("terminal.ansi.red", json!(c.red)),
        ("terminal.ansi.bright_red", json!("#ff4d6a")),
        ("terminal.ansi.dim_red", json!(dim(c.red))),
        ("terminal.ansi.green", json!(v.green)),
        ("terminal.ansi.bright_green", json!(v.bright_green)),
        ("terminal.ansi.dim_green", json!(dim(v.green))),
        ("terminal.ansi.yellow", json!(v.lime)),
        ("terminal.ansi.bright_yellow", json!(c.yellow)),
        ("terminal.ansi.dim_yellow", json!(dim(v.lime))),
        ("terminal.ansi.blue", json!(v.blue)),
        ("terminal.ansi.bright_blue", json!(v.teal)),
        ("terminal.ansi.dim_blue", json!(dim(v.blue))),
        ("terminal.ansi.magenta", json!(c.magenta)),
        ("terminal.ansi.bright_magenta", json!("#c040e0")),
        ("terminal.ansi.dim_magenta", json!(dim(c.magenta))),
        ("terminal.ansi.cyan", json!(v.cyan)),
        ("terminal.ansi.bright_cyan", json!("#00eeff")),
        ("terminal.ansi.dim_cyan", json!(dim(v.cyan))),
        ("terminal.ansi.white", json!(c.fg)),
        ("terminal.ansi.bright_white", json!("#ffffff")),
        ("terminal.ansi.dim_white", json!(c.muted)),
        // Version control
        ("version_control.added", json!(v.lime)),
        ("version_control.modified", json!(c.orange)),
        ("version_control.deleted", json!(c.red)),
        ("version_control.renamed", json!(v.teal)),
        ("version_control.conflict", json!(c.yellow)),
        ("version_control.ignored", json!(c.line)),
        ("version_control.word_added", json!(a(v.lime, 0x33))),
        ("version_control.word_deleted", json!(a(c.red, 0x40))),
        ("version_control.conflict_marker.ours", json!(a(v.lime, 0x1a))),
        ("version_control.conflict_marker.theirs", json!(a(v.blue, 0x26))),
    ]);

    // Diagnostic / status colors
    let statuses = [
        ("conflict", c.yellow),
        ("created", v.lime),
        ("deleted", c.red),
        ("error", c.red),
        ("hidden", c.comment),
        ("hint", c.comment),
        ("ignored", c.line),
        ("info", v.teal),
        ("modified", c.orange),
        ("predictive", c.comment),
        ("renamed", v.teal),
        ("success", v.lime),
        ("unreachable", c.comment),
        ("warning", c.orange),
    ];
    for (key, color) in statuses {
        style.insert(key.into(), json!(color));
        style.insert(format!("{key}.background"), json!(a(color, 0x1a)));
        style.insert(format!("{key}.border"), json!(a(color, 0x55)));
    }

    // Rainbow brackets / indent guides
    style.insert(
        "accents".into(),
        json!([v.teal, v.blue, v.green, v.lime, v.cyan, c.orange]),
    );

    let mut players = vec![json!({ "cursor": c.fg, "background": c.fg, "selection": selection })];
    for color in [v.teal, v.green, c.orange, c.magenta, v.lime, c.red, c.yellow] {
        players.push(json!({ "cursor": color, "background": color, "selection": a(color, 0x3d) }));
    }
    style.insert("players".into(), Value::Array(players));

    // Syntax: mirrors the VS Code tokenColors where Zed has an equivalent.
    let syntax = object(vec![
        ("comment", s(c.comment)),
        ("comment.doc", s(c.comment)),
        ("string", s(v.string)),
        ("string.escape", s(v.teal)),
        ("string.regex", s(v.cyan)),
        ("string.special", s(v.teal)),
        ("string.special.symbol", s(v.green)),
        ("number", s(v.teal)),
        ("boolean", s(v.green)),
        ("constant", s(v.green)),
        ("constant.builtin", s(v.green)),
        ("variable", s(c.fg)),
        ("variable.parameter", s(c.fg)),
        ("variable.special", italic(v.green)),
        ("variable.member", s(c.fg)),
        ("keyword", s(v.blue)),
        ("keyword.modifier", italic(v.green)),
        ("operator", s(v.blue)),
        ("function", s(v.teal)),
        ("function.builtin", s(v.green)),
        ("function.method", s(v.teal)),
        ("constructor", s(v.green)),
        ("type", s(v.green)),
        ("type.builtin", s(v.green)),
        ("type.interface", s(v.green)),
        ("enum", s(v.green)),
        ("variant", s(v.green)),
        ("namespace", s(v.green)),
        ("lifetime", italic(v.green)),
        ("label", s(v.blue)),
        ("preproc", s(v.blue)),
        ("attribute", s(v.green)),
        ("property", s(c.fg)),
        ("property.json_key", s(v.green)),
        ("tag", s(v.blue)),
        ("selector", s(v.blue)),
        ("selector.pseudo", s(v.green)),
        ("punctuation", s(c.fg)),
        ("punctuation.bracket", s(c.muted)),
        ("punctuation.delimiter", s(c.muted)),
        ("punctuation.list_marker", s(v.blue)),
        ("punctuation.markup", s(v.blue)),
        ("punctuation.special", s(v.blue)),
        ("embedded", s(c.fg)),
        ("primary", s(c.fg)),
        ("title", bold(v.green)),
        ("text.literal", s(v.teal)),
        ("emphasis", italic(v.blue)),
        ("emphasis.strong", bold(v.blue)),
        ("link_text", s(v.blue)),
        ("link_uri", italic(v.teal)),
        ("hint", italic(c.comment)),
        ("predictive", italic(mix(c.comment, c.bg, 0.75))),
        ("diff.plus", s(v.lime)),
        ("diff.minus", s(c.red)),
    ]);
    style.insert("syntax".into(), Value::Object(syntax));

    json!({ "name": v.name, "appearance": "dark", "style": style })
}

fn main() {
    let family = json!({
        "$schema": "https://zed.dev/schema/themes/v0.2.0.json",
        "name": "Black Ocean",
        "author": "mustzev (Zed port); Alex Oxthorn (original VS Code theme)",
        "themes": VARIANTS.iter().map(theme).collect::<Vec<_>>(),
    });

    let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("../themes/black-ocean.json");
    let mut text = serde_json::to_string_pretty(&family).unwrap();
    text.push('\n');
    std::fs::write(&out, text).expect("failed to write theme");
    println!("wrote {}", out.display());
}
