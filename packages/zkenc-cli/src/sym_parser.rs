/// Symbol file parser for Circom .sym files
///
/// The .sym file format is:
/// <label_id>,<wire_id>,<component_id>,<signal_name>
///
/// Example:
/// 1,1,172,main.root
/// 2,2,172,main.message
/// 3,3,172,main.R8x
use anyhow::Result;
use std::collections::HashMap;

/// Parse Circom .sym file to get signal name to wire ID mapping
///
/// # Arguments
/// * `sym_content` - Content of the .sym file as string
///
/// # Returns
/// Map from signal name to wire ID
pub fn parse_sym_file(sym_content: &str) -> Result<HashMap<String, u32>> {
    let mut wire_map = HashMap::new();

    for (line_num, line) in sym_content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 4 {
            continue; // Skip invalid lines
        }

        // Parse wire_id as i32 first to handle negative values
        let wire_id_signed: i32 = parts[1].trim().parse()
            .map_err(|e| anyhow::anyhow!("Line {}: Failed to parse wire_id '{}': {}", line_num + 1, parts[1], e))?;
        
        // Only include signals with valid wire IDs (wireId >= 0)
        // Wire ID -1 means internal signal, not an input/output
        if wire_id_signed < 0 {
            continue;
        }
        
        let wire_id = wire_id_signed as u32;
        let signal_name = parts[3].trim().to_string();

        wire_map.insert(signal_name, wire_id);
    }

    Ok(wire_map)
}

/// Extract input signals from wire mapping
/// Filters for signals that start with "main." and are inputs
///
/// # Arguments
/// * `wire_map` - Wire mapping from parse_sym_file
/// * `max_wire_id` - Maximum wire ID to consider as input (typically n_pub_in + n_pub_out)
///
/// # Returns
/// Map from simplified signal name (without "main.") to wire ID
pub fn get_input_signals(
    wire_map: &HashMap<String, u32>,
    max_wire_id: Option<u32>,
) -> HashMap<String, u32> {
    let mut input_signals = HashMap::new();

    for (signal_name, &wire_id) in wire_map.iter() {
        // Skip if wireId exceeds maximum (if specified)
        if let Some(max) = max_wire_id {
            if wire_id > max {
                continue;
            }
        }

        // Extract input signals (those starting with "main.")
        if let Some(simplified) = signal_name.strip_prefix("main.") {
            input_signals.insert(simplified.to_string(), wire_id);
        }
    }

    input_signals
}
