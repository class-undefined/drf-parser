use std::{collections::HashMap, error::Error, fs, io, path::Path};

use serde::Serialize;

use crate::core::ast::{
    color::Color, drf::Drf, line_style::LineStyle, packet::Packet, stipple::Stipple,
};

#[derive(Debug, Clone)]
struct LayerProperties {
    name: String,
    source: String,
    frame_color: String,
    fill_color: String,
    dither_pattern: String,
    line_style: String,
}

#[derive(Debug, Default)]
struct CustomDitherPattern {
    lines: Vec<String>,
}

#[derive(Debug, Default)]
struct CustomLineStyle {
    pattern: String,
}

#[derive(Debug, Serialize)]
pub struct LypOutput {
    pub drf: HashMap<String, Drf>,
    pub layermap: HashMap<String, (usize, usize)>,
}

impl LypOutput {
    pub fn drf_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.drf)
    }

    pub fn layermap_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.layermap)
    }
}

pub fn parse_lyp_file(path: &str) -> Result<LypOutput, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    parse_lyp(&content)
}

pub fn write_lyp_outputs(path: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    let output = parse_lyp_file(path)?;
    fs::create_dir_all(output_dir)?;
    fs::write(
        Path::new(output_dir).join("display.drf.json"),
        output.drf_json()?,
    )?;
    fs::write(
        Path::new(output_dir).join("layermap.json"),
        output.layermap_json()?,
    )?;
    Ok(())
}

pub fn parse_lyp(content: &str) -> Result<LypOutput, Box<dyn Error>> {
    let mut properties = Vec::new();
    let mut dither_patterns = Vec::new();
    let mut line_styles = Vec::new();

    let mut current_property: Option<LayerProperties> = None;
    let mut current_dither: Option<CustomDitherPattern> = None;
    let mut current_line_style: Option<CustomLineStyle> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        match trimmed {
            "<properties>" => {
                current_property = Some(LayerProperties {
                    name: String::new(),
                    source: String::new(),
                    frame_color: String::new(),
                    fill_color: String::new(),
                    dither_pattern: String::new(),
                    line_style: String::new(),
                });
                continue;
            }
            "</properties>" => {
                if let Some(property) = current_property.take() {
                    properties.push(property);
                }
                continue;
            }
            "<custom-dither-pattern>" => {
                current_dither = Some(CustomDitherPattern::default());
                continue;
            }
            "</custom-dither-pattern>" => {
                if let Some(pattern) = current_dither.take() {
                    dither_patterns.push(pattern);
                }
                continue;
            }
            "<custom-line-style>" => {
                current_line_style = Some(CustomLineStyle::default());
                continue;
            }
            "</custom-line-style>" => {
                if let Some(style) = current_line_style.take() {
                    line_styles.push(style);
                }
                continue;
            }
            _ => {}
        }

        if let Some(property) = current_property.as_mut() {
            if let Some(value) = element_text(trimmed, "name") {
                property.name = value;
            } else if let Some(value) = element_text(trimmed, "source") {
                property.source = value;
            } else if let Some(value) = element_text(trimmed, "frame-color") {
                property.frame_color = value;
            } else if let Some(value) = element_text(trimmed, "fill-color") {
                property.fill_color = value;
            } else if let Some(value) = element_text(trimmed, "dither-pattern") {
                property.dither_pattern = value;
            } else if let Some(value) = element_text(trimmed, "line-style") {
                property.line_style = value;
            }
            continue;
        }

        if let Some(pattern) = current_dither.as_mut() {
            if let Some(value) = element_text(trimmed, "line") {
                pattern.lines.push(value);
            }
            continue;
        }

        if let Some(style) = current_line_style.as_mut() {
            if let Some(value) = element_text(trimmed, "pattern") {
                style.pattern = value;
            }
        }
    }

    build_output(properties, dither_patterns, line_styles)
}

fn build_output(
    properties: Vec<LayerProperties>,
    dither_patterns: Vec<CustomDitherPattern>,
    line_styles: Vec<CustomLineStyle>,
) -> Result<LypOutput, Box<dyn Error>> {
    let mut display = Drf::new(String::from("display"));
    let mut layermap = HashMap::new();

    insert_builtin_stipples(&mut display);
    insert_builtin_line_styles(&mut display);

    for (index, pattern) in dither_patterns.into_iter().enumerate() {
        if pattern.lines.is_empty() {
            continue;
        }
        let bitmap = pattern
            .lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|ch| if ch == '*' { 1 } else { 0 })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
        let name = klayout_custom_name(index);
        let row = bitmap.len();
        let col = bitmap.first().map_or(0, Vec::len);
        display
            .stipples
            .insert(name.clone(), Stipple::new(name, bitmap, row, col));
    }

    for (index, style) in line_styles.into_iter().enumerate() {
        let name = klayout_custom_name(index);
        let pattern = style
            .pattern
            .chars()
            .map(|ch| if ch == '*' { 1 } else { 0 })
            .collect::<Vec<u8>>();
        display
            .line_styles
            .insert(name.clone(), LineStyle::new(name, 1, pattern));
    }

    for property in properties {
        let Some((layer_name, purpose)) = parse_layer_purpose(&property.name) else {
            continue;
        };
        let Some((layer_number, purpose_number)) = parse_source(&property.source) else {
            continue;
        };

        layermap.insert(
            format!("{}#{}", layer_name, purpose),
            (layer_number, purpose_number),
        );

        let fill = color_name(&property.fill_color);
        let outline = color_name(&property.frame_color);
        insert_color(&mut display, &fill, &property.fill_color);
        insert_color(&mut display, &outline, &property.frame_color);

        let packet_name = packet_name(&layer_name, &purpose);
        let stipple = normalize_klayout_ref(&property.dither_pattern, ReferenceKind::Dither);
        let line_style = normalize_klayout_ref(&property.line_style, ReferenceKind::LineStyle);
        display.packets.insert(
            packet_name.clone(),
            Packet {
                name: packet_name,
                stipple,
                line_style,
                fill,
                outline,
                fill_style: None,
            },
        );
    }

    let mut drf = HashMap::new();
    drf.insert(display.name.clone(), display);
    let output = LypOutput { drf, layermap };
    validate_output(&output)?;
    Ok(output)
}

fn insert_builtin_stipples(display: &mut Drf) {
    let solid = vec![vec![1; 16]; 16];
    let clear = vec![vec![0; 16]; 16];
    display.stipples.insert(
        String::from("I0"),
        Stipple::new(String::from("I0"), solid, 16, 16),
    );
    display.stipples.insert(
        String::from("I1"),
        Stipple::new(String::from("I1"), clear, 16, 16),
    );
}

fn insert_builtin_line_styles(display: &mut Drf) {
    display.line_styles.insert(
        String::from("I0"),
        LineStyle::new(String::from("I0"), 1, vec![1]),
    );
    display.line_styles.insert(
        String::from("I1"),
        LineStyle::new(String::from("I1"), 1, vec![1, 0]),
    );
}

fn insert_color(display: &mut Drf, name: &str, value: &str) {
    if display.colors.contains_key(name) {
        return;
    }
    if let Some([r, g, b]) = parse_hex_color(value) {
        display.colors.insert(
            name.to_string(),
            Color::new(name.to_string(), r, g, b, false),
        );
    }
}

fn parse_layer_purpose(name: &str) -> Option<(String, String)> {
    let before_dash = name.split(" - ").next()?.trim();
    let (layer, purpose) = before_dash.split_once('.')?;
    Some((layer.trim().to_string(), purpose.trim().to_string()))
}

fn parse_source(source: &str) -> Option<(usize, usize)> {
    let before_at = source.split('@').next()?.trim();
    let (layer, datatype) = before_at.split_once('/')?;
    Some((layer.parse().ok()?, datatype.parse().ok()?))
}

#[derive(Clone, Copy)]
enum ReferenceKind {
    Dither,
    LineStyle,
}

fn normalize_klayout_ref(value: &str, kind: ReferenceKind) -> String {
    let value = value.trim();
    if value.is_empty() {
        return match kind {
            ReferenceKind::Dither => String::from("I0"),
            ReferenceKind::LineStyle => String::from("I0"),
        };
    }

    if matches!(value.as_bytes().first(), Some(b'I' | b'C')) {
        return value.to_string();
    }

    if let Ok(index) = value.parse::<usize>() {
        if index < 16 {
            return format!("I{}", index);
        }
        return format!("C{}", index - 16);
    }

    value.to_string()
}

fn packet_name(layer_name: &str, purpose: &str) -> String {
    format!("{}_{}", layer_name, purpose)
}

fn parse_hex_color(value: &str) -> Option<[u8; 3]> {
    let hex = value.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    Some([
        u8::from_str_radix(&hex[0..2], 16).ok()?,
        u8::from_str_radix(&hex[2..4], 16).ok()?,
        u8::from_str_radix(&hex[4..6], 16).ok()?,
    ])
}

fn color_name(value: &str) -> String {
    format!(
        "color_{}",
        value.trim_start_matches('#').to_ascii_lowercase()
    )
}

fn klayout_custom_name(index: usize) -> String {
    format!("C{}", index)
}

fn element_text(line: &str, tag: &str) -> Option<String> {
    let start = format!("<{}>", tag);
    let end = format!("</{}>", tag);
    let value = line.strip_prefix(&start)?.strip_suffix(&end)?;
    Some(decode_xml(value))
}

fn decode_xml(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn validate_output(output: &LypOutput) -> Result<(), Box<dyn Error>> {
    for (display_name, display) in &output.drf {
        for (name, stipple) in &display.stipples {
            if stipple.row != stipple.bitmap.len() {
                return Err(invalid_data(format!(
                    "{display_name}.{name} row mismatch: row={}, bitmap rows={}",
                    stipple.row,
                    stipple.bitmap.len()
                )));
            }
            if stipple.bitmap.iter().any(|row| row.len() != stipple.col) {
                return Err(invalid_data(format!(
                    "{display_name}.{name} has non-rectangular bitmap"
                )));
            }
        }

        for (name, packet) in &display.packets {
            if !display.stipples.contains_key(&packet.stipple) {
                return Err(invalid_data(format!(
                    "{display_name}.{name} references missing stipple {}",
                    packet.stipple
                )));
            }
            if !display.line_styles.contains_key(&packet.line_style) {
                return Err(invalid_data(format!(
                    "{display_name}.{name} references missing line_style {}",
                    packet.line_style
                )));
            }
            if !display.colors.contains_key(&packet.fill) {
                return Err(invalid_data(format!(
                    "{display_name}.{name} references missing fill color {}",
                    packet.fill
                )));
            }
            if !display.colors.contains_key(&packet.outline) {
                return Err(invalid_data(format!(
                    "{display_name}.{name} references missing outline color {}",
                    packet.outline
                )));
            }
        }
    }

    Ok(())
}

fn invalid_data(message: String) -> Box<dyn Error> {
    io::Error::new(io::ErrorKind::InvalidData, message).into()
}

#[cfg(test)]
mod tests {
    use super::parse_lyp;

    #[test]
    fn parses_layer_properties_into_drf_and_layermap() {
        let content = r##"
<?xml version="1.0" encoding="utf-8"?>
<layer-properties>
 <properties>
  <frame-color>#ff0000</frame-color>
  <fill-color>#00ff00</fill-color>
  <dither-pattern>C0</dither-pattern>
  <line-style>16</line-style>
  <name>M1.drawing - 61/0</name>
  <source>61/0@1</source>
 </properties>
 <properties>
  <frame-color>#ff0000</frame-color>
  <fill-color>#00ff00</fill-color>
  <dither-pattern>I1</dither-pattern>
  <line-style>17</line-style>
  <name>M2.drawing - 62/0</name>
  <source>62/0@1</source>
 </properties>
 <custom-dither-pattern>
  <pattern>
   <line>*.</line>
   <line>.*</line>
  </pattern>
  <order>99</order>
  <name>checker</name>
 </custom-dither-pattern>
 <custom-dither-pattern>
  <pattern>
   <line>..</line>
   <line>..</line>
  </pattern>
  <order>1</order>
  <name>blank</name>
 </custom-dither-pattern>
 <custom-line-style>
  <pattern>***.</pattern>
  <order>99</order>
  <name>dashed</name>
 </custom-line-style>
 <custom-line-style>
  <pattern>*</pattern>
  <order>1</order>
  <name>solid</name>
 </custom-line-style>
</layer-properties>
"##;

        let output = parse_lyp(content).unwrap();
        assert_eq!(output.layermap.get("M1#drawing"), Some(&(61, 0)));
        assert_eq!(output.layermap.get("M2#drawing"), Some(&(62, 0)));

        let display = output.drf.get("display").unwrap();
        assert!(display.colors.contains_key("color_ff0000"));
        assert!(display.colors.contains_key("color_00ff00"));
        assert!(display.stipples.contains_key("C0"));
        assert!(display.stipples.contains_key("I0"));
        assert!(display.stipples.contains_key("I1"));
        assert!(display.line_styles.contains_key("C0"));
        assert_eq!(display.stipples.get("C0").unwrap().bitmap[0], vec![1, 0]);
        assert_eq!(display.stipples.get("I0").unwrap().bitmap[0], vec![1; 16]);
        assert_eq!(display.stipples.get("I1").unwrap().bitmap[0], vec![0; 16]);
        assert_eq!(display.stipples.get("C1").unwrap().bitmap[0], vec![0, 0]);
        assert_eq!(
            display.line_styles.get("C0").unwrap().pattern,
            vec![1, 1, 1, 0]
        );
        assert_eq!(display.line_styles.get("C1").unwrap().pattern, vec![1]);

        let packet = display.packets.get("M1_drawing").unwrap();
        assert_eq!(packet.name, "M1_drawing");
        assert_eq!(packet.outline, "color_ff0000");
        assert_eq!(packet.fill, "color_00ff00");
        assert_eq!(packet.stipple, "C0");
        assert_eq!(packet.line_style, "C0");

        let builtin_packet = display.packets.get("M2_drawing").unwrap();
        assert_eq!(builtin_packet.stipple, "I1");
        assert_eq!(builtin_packet.line_style, "C1");
    }
}
