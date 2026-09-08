mod common;

use common::{contrast_ratio, extract_hex_colors, hex_to_lower, is_valid_hex};
use serde_norway::Value;

const THEME: &str = include_str!("../polytoken/warm-burnout.yaml");

fn parse_theme() -> Value {
  serde_norway::from_str(THEME).expect("invalid YAML")
}

/// Resolve a `$ref: "#/palette/..."` pointer to its palette color value.
/// Palette leaves are `{rgb: "#..."}` maps; unwrap the `rgb` field.
fn resolve_ref<'a>(v: &'a Value, r: &str) -> &'a Value {
  let path = r.strip_prefix("#/").expect("$ref must start with '#/'");
  let mut node = v;
  for part in path.split('/') {
    node = &node[part];
  }
  node.get("rgb").unwrap_or(node)
}

fn token_ref(v: &Value, variant: &str, token: &str, prop: &str) -> String {
  let style = &v[variant][token];
  assert!(style.as_mapping().is_some(), "missing token '{variant}.{token}'");
  let ref_str = style[prop]["$ref"]
    .as_str()
    .unwrap_or_else(|| panic!("token '{variant}.{token}' has no $ref {prop}"));
  hex_to_lower(resolve_ref(v, ref_str).as_str().expect("ref target not a string"))
}

fn token_fg(v: &Value, variant: &str, token: &str) -> String {
  token_ref(v, variant, token, "fg")
}

fn token_bg(v: &Value, variant: &str, token: &str) -> String {
  token_ref(v, variant, token, "bg")
}

fn token_bg_raw(v: &Value, variant: &str, token: &str) -> String {
  hex_to_lower(
    v[variant][token]["bg"]["rgb"]
      .as_str()
      .unwrap_or_else(|| panic!("token '{variant}.{token}' has no raw rgb bg")),
  )
}

// -- Valid YAML --

#[test]
fn is_valid_yaml() {
  parse_theme();
}

// -- Schema structure --

#[test]
fn has_version_field() {
  assert_eq!(parse_theme()["version"].as_i64(), Some(1));
}

#[test]
fn has_title_field() {
  assert_eq!(parse_theme()["title"].as_str(), Some("Warm Burnout"));
}

// -- Both variants present --

#[test]
fn has_dark_and_light_variants() {
  let v = parse_theme();
  assert!(v["dark"].as_mapping().is_some(), "missing 'dark' token section");
  assert!(v["light"].as_mapping().is_some(), "missing 'light' token section");
}

// -- Required base tokens --

#[test]
fn body_and_app_background_defined_in_both_variants() {
  let v = parse_theme();
  for variant in ["dark", "light"] {
    assert!(v[variant]["body"].as_mapping().is_some(), "{variant} missing 'body'");
    assert!(
      v[variant]["app_background"].as_mapping().is_some(),
      "{variant} missing 'app_background'"
    );
  }
}

// -- Canonical backgrounds --

#[test]
fn dark_background_is_canonical() {
  assert_eq!(token_bg(&parse_theme(), "dark", "app_background"), "#1a1510");
}

#[test]
fn light_background_is_canonical() {
  assert_eq!(token_bg(&parse_theme(), "light", "app_background"), "#f5ede0");
}

// -- Canonical foregrounds --

#[test]
fn dark_foreground_is_canonical() {
  assert_eq!(token_fg(&parse_theme(), "dark", "body"), "#bfbdb6");
}

#[test]
fn light_foreground_is_canonical() {
  assert_eq!(token_fg(&parse_theme(), "light", "body"), "#3a3630");
}

// -- Syntax colors match canonical palette --

#[test]
fn dark_syntax_colors_match_palette() {
  let v = parse_theme();
  let expected = [
    ("code_keyword", "#ff8f40"),
    ("code_string", "#b4bc78"),
    ("code_comment", "#b4a89c"),
    ("code_literal", "#d4a8b8"),
    ("code_type", "#90aec0"),
    ("code_function", "#ffb454"),
    ("code_operator", "#f29668"),
    ("code_number", "#d4a8b8"),
  ];
  for (token, color) in expected {
    assert_eq!(token_fg(&v, "dark", token), color, "dark {token} mismatch");
  }
}

#[test]
fn light_syntax_colors_match_palette() {
  let v = parse_theme();
  let expected = [
    ("code_keyword", "#924800"),
    ("code_string", "#4d5c1a"),
    ("code_comment", "#544c40"),
    ("code_literal", "#7e4060"),
    ("code_type", "#285464"),
    ("code_function", "#855700"),
    ("code_operator", "#8f4418"),
    ("code_number", "#7e4060"),
  ];
  for (token, color) in expected {
    assert_eq!(token_fg(&v, "light", token), color, "light {token} mismatch");
  }
}

// -- Font styles preserved (three-tier font system) --

#[test]
fn dark_keyword_is_bold() {
  assert_eq!(parse_theme()["dark"]["code_keyword"]["b"].as_bool(), Some(true));
}

#[test]
fn dark_comment_is_italic() {
  assert_eq!(parse_theme()["dark"]["code_comment"]["i"].as_bool(), Some(true));
}

#[test]
fn dark_type_is_italic() {
  assert_eq!(parse_theme()["dark"]["code_type"]["i"].as_bool(), Some(true));
}

#[test]
fn light_keyword_is_bold() {
  assert_eq!(parse_theme()["light"]["code_keyword"]["b"].as_bool(), Some(true));
}

#[test]
fn light_comment_is_italic() {
  assert_eq!(parse_theme()["light"]["code_comment"]["i"].as_bool(), Some(true));
}

#[test]
fn light_type_is_italic() {
  assert_eq!(parse_theme()["light"]["code_type"]["i"].as_bool(), Some(true));
}

// -- All hex colors valid --

#[test]
fn all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(THEME) {
    assert!(is_valid_hex(hex), "line {line}: invalid hex: {hex}");
  }
}

// -- No pure black/white backgrounds --

#[test]
fn no_pure_black_background() {
  let bg = token_bg(&parse_theme(), "dark", "app_background");
  assert_ne!(bg, "#000000", "dark background must not be pure black");
}

#[test]
fn no_pure_white_background() {
  let bg = token_bg(&parse_theme(), "light", "app_background");
  assert_ne!(bg, "#ffffff", "light background must not be pure white");
}

// -- WCAG contrast: syntax + semantic tokens against the app background --

fn contrast_checked_tokens() -> Vec<&'static str> {
  vec![
    "body",
    "error",
    "warning",
    "success",
    "info",
    "code_keyword",
    "code_string",
    "code_comment",
    "code_literal",
    "code_type",
    "code_function",
    "code_operator",
    "code_number",
    "code_punctuation",
    "status_bar_model",
    "status_bar_label",
  ]
}

#[test]
fn dark_syntax_tokens_meet_aaa() {
  let v = parse_theme();
  let bg = token_bg(&v, "dark", "app_background");
  for token in contrast_checked_tokens() {
    let fg = token_fg(&v, "dark", token);
    let ratio = contrast_ratio(&fg, &bg);
    assert!(
      ratio >= 7.0,
      "dark {token} {fg} has contrast {ratio:.2}:1, needs >= 7.0:1"
    );
  }
}

#[test]
fn light_syntax_tokens_meet_aa() {
  let v = parse_theme();
  let bg = token_bg(&v, "light", "app_background");
  for token in contrast_checked_tokens() {
    let fg = token_fg(&v, "light", token);
    let ratio = contrast_ratio(&fg, &bg);
    assert!(
      ratio >= 4.5,
      "light {token} {fg} has contrast {ratio:.2}:1, needs >= 4.5:1"
    );
  }
}

// -- Every $ref resolves within its own variant --

#[test]
fn all_refs_resolve_and_stay_in_variant() {
  let v = parse_theme();
  for variant in ["dark", "light"] {
    let variant_obj = v[variant]
      .as_mapping()
      .unwrap_or_else(|| panic!("variant '{variant}' is not a mapping"));
    assert!(!variant_obj.is_empty(), "variant '{variant}' has no tokens");
    for (key, style) in variant_obj {
      let token = key
        .as_str()
        .unwrap_or_else(|| panic!("token key in '{variant}' is not a string"));
      let obj = style
        .as_mapping()
        .unwrap_or_else(|| panic!("token '{variant}.{token}' is not a mapping"));
      for (prop_key, prop_val) in obj {
        let prop = prop_key.as_str().unwrap_or("");
        let Some(r) = prop_val.get("$ref").and_then(|r| r.as_str()) else {
          continue;
        };
        assert!(
          r.starts_with(&format!("#/palette/{variant}/")),
          "token '{variant}.{token}' {prop} escapes its variant: {r}"
        );
        assert!(
          resolve_ref(&v, r).is_string(),
          "token '{variant}.{token}' {prop} ref does not resolve to a string: {r}"
        );
      }
    }
  }
}

// -- Both variants define the same token set --

#[test]
fn both_variants_define_same_tokens() {
  let v = parse_theme();
  let dark_keys: Vec<&str> = v["dark"]
    .as_mapping()
    .unwrap()
    .keys()
    .filter_map(|k| k.as_str())
    .collect();
  let light_keys: Vec<&str> = v["light"]
    .as_mapping()
    .unwrap()
    .keys()
    .filter_map(|k| k.as_str())
    .collect();
  for key in &dark_keys {
    assert!(light_keys.contains(key), "dark has token '{key}' but light does not");
  }
  for key in &light_keys {
    assert!(dark_keys.contains(key), "light has token '{key}' but dark does not");
  }
}

// -- Selection backgrounds are the Ghostty neutrals --

#[test]
fn dark_selection_is_ghostty_neutral() {
  let v = parse_theme();
  assert_eq!(token_bg(&v, "dark", "selected"), "#33393a");
  assert_eq!(token_bg(&v, "dark", "prompt_selection"), "#33393a");
}

#[test]
fn light_selection_is_ghostty_neutral() {
  let v = parse_theme();
  assert_eq!(token_bg(&v, "light", "selected"), "#e5e8e2");
  assert_eq!(token_bg(&v, "light", "prompt_selection"), "#e5e8e2");
}

// -- Raw (non-$ref) backgrounds only exist for mode tints --

#[test]
fn raw_mode_tints_are_opaque_tints() {
  let v = parse_theme();
  for (variant, expected) in [
    ("dark", ["#33201a", "#452a20", "#22282c"]),
    ("light", ["#ecdcd2", "#e4c9bc", "#dde3e1"]),
  ] {
    assert_eq!(token_bg_raw(&v, variant, "command_mode_background"), expected[0]);
    assert_eq!(token_bg_raw(&v, variant, "command_mode_background_bright"), expected[1]);
    assert_eq!(token_bg_raw(&v, variant, "slash_mode_background"), expected[2]);
  }
}
