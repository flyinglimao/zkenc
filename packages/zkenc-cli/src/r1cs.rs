use anyhow::{bail, Context, Result};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::Path;

/// R1CS file parser
/// 
/// Parses Circom R1CS binary format according to:
/// https://github.com/iden3/r1csfile/blob/master/doc/r1cs_bin_format.md
#[derive(Debug)]
pub struct R1csFile {
    pub field_size: u32,
    pub prime: Vec<u8>,
    pub n_wires: u32,
    pub n_pub_out: u32,
    pub n_pub_in: u32,
    pub n_prv_in: u32,
    pub n_labels: u64,
    pub n_constraints: u32,
    pub constraints: Vec<Constraint>,
    pub wire2label: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub a: LinearCombination,
    pub b: LinearCombination,
    pub c: LinearCombination,
}

#[derive(Debug, Clone)]
pub struct LinearCombination {
    pub factors: Vec<(u32, Vec<u8>)>, // (wire_id, value in little-endian)
}

impl R1csFile {
    /// Parse an R1CS file from disk
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path.as_ref())
            .with_context(|| format!("Failed to open R1CS file: {:?}", path.as_ref()))?;
        let mut reader = BufReader::new(file);

        // Parse magic number "r1cs"
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        if &magic != b"r1cs" {
            bail!("Invalid R1CS file: wrong magic number {:?}", magic);
        }

        // Parse version (must be 1)
        let version = read_u32(&mut reader)?;
        if version != 1 {
            bail!("Unsupported R1CS version: {}", version);
        }

        // Parse number of sections
        let n_sections = read_u32(&mut reader)?;

        // First pass: collect all section positions
        let mut sections = Vec::new();
        for _ in 0..n_sections {
            let section_type = read_u32(&mut reader)?;
            let section_size = read_u64(&mut reader)?;
            let section_pos = reader.stream_position()?;
            sections.push((section_type, section_size, section_pos));
            reader.seek_relative(section_size as i64)?;
        }

        // Second pass: parse header first
        let header = {
            let header_section = sections.iter()
                .find(|(t, _, _)| *t == 0x01)
                .context("Header section not found")?;
            reader.seek(std::io::SeekFrom::Start(header_section.2))?;
            Self::parse_header(&mut reader)?
        };

        // Parse constraints section
        let constraints = {
            let constraints_section = sections.iter()
                .find(|(t, _, _)| *t == 0x02)
                .context("Constraints section not found")?;
            reader.seek(std::io::SeekFrom::Start(constraints_section.2))?;
            Self::parse_constraints(
                &mut reader,
                header.n_constraints as usize,
                header.field_size as usize,
            )?
        };

        // Parse wire2label section (optional)
        let wire2label = if let Some(wire2label_section) = sections.iter().find(|(t, _, _)| *t == 0x03) {
            reader.seek(std::io::SeekFrom::Start(wire2label_section.2))?;
            Self::parse_wire2label(&mut reader, header.n_wires as usize)?
        } else {
            // Generate default wire2label if not present
            (0..header.n_wires as u64).collect()
        };

        Ok(R1csFile {
            field_size: header.field_size,
            prime: header.prime,
            n_wires: header.n_wires,
            n_pub_out: header.n_pub_out,
            n_pub_in: header.n_pub_in,
            n_prv_in: header.n_prv_in,
            n_labels: header.n_labels,
            n_constraints: header.n_constraints,
            constraints,
            wire2label,
        })
    }

    fn parse_header(reader: &mut BufReader<File>) -> Result<R1csHeader> {
        let field_size = read_u32(reader)?;
        let mut prime = vec![0u8; field_size as usize];
        reader.read_exact(&mut prime)?;

        let n_wires = read_u32(reader)?;
        let n_pub_out = read_u32(reader)?;
        let n_pub_in = read_u32(reader)?;
        let n_prv_in = read_u32(reader)?;
        let n_labels = read_u64(reader)?;
        let n_constraints = read_u32(reader)?;

        Ok(R1csHeader {
            field_size,
            prime,
            n_wires,
            n_pub_out,
            n_pub_in,
            n_prv_in,
            n_labels,
            n_constraints,
        })
    }

    fn parse_constraints(
        reader: &mut BufReader<File>,
        n_constraints: usize,
        field_size: usize,
    ) -> Result<Vec<Constraint>> {
        let mut constraints = Vec::with_capacity(n_constraints);

        for _ in 0..n_constraints {
            let a = Self::parse_lc(reader, field_size)?;
            let b = Self::parse_lc(reader, field_size)?;
            let c = Self::parse_lc(reader, field_size)?;

            constraints.push(Constraint { a, b, c });
        }

        Ok(constraints)
    }

    fn parse_lc(reader: &mut BufReader<File>, field_size: usize) -> Result<LinearCombination> {
        let n_factors = read_u32(reader)?;
        let mut factors = Vec::with_capacity(n_factors as usize);

        for _ in 0..n_factors {
            let wire_id = read_u32(reader)?;
            let mut value = vec![0u8; field_size];
            reader.read_exact(&mut value)?;
            factors.push((wire_id, value));
        }

        Ok(LinearCombination { factors })
    }

    fn parse_wire2label(reader: &mut BufReader<File>, n_wires: usize) -> Result<Vec<u64>> {
        let mut wire2label = Vec::with_capacity(n_wires);
        for _ in 0..n_wires {
            wire2label.push(read_u64(reader)?);
        }
        Ok(wire2label)
    }

    /// Get the number of public inputs (outputs + inputs)
    pub fn n_public_inputs(&self) -> u32 {
        self.n_pub_out + self.n_pub_in
    }
}

#[derive(Debug)]
struct R1csHeader {
    field_size: u32,
    prime: Vec<u8>,
    n_wires: u32,
    n_pub_out: u32,
    n_pub_in: u32,
    n_prv_in: u32,
    n_labels: u64,
    n_constraints: u32,
}

// Helper functions for reading little-endian integers
fn read_u32(reader: &mut BufReader<File>) -> Result<u32> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64(reader: &mut BufReader<File>) -> Result<u64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_signature_r1cs() {
        let r1cs_path = PathBuf::from("tests/r1cs/signature.r1cs");
        let r1cs = R1csFile::from_file(&r1cs_path).expect("Failed to parse R1CS");

        println!("R1CS Info:");
        println!("  Field size: {} bytes", r1cs.field_size);
        println!("  Prime (hex): 0x{}", hex::encode(&r1cs.prime));
        println!("  Wires: {}", r1cs.n_wires);
        println!("  Public outputs: {}", r1cs.n_pub_out);
        println!("  Public inputs: {}", r1cs.n_pub_in);
        println!("  Private inputs: {}", r1cs.n_prv_in);
        println!("  Labels: {}", r1cs.n_labels);
        println!("  Constraints: {}", r1cs.n_constraints);
        
        assert!(r1cs.n_wires > 0, "Should have wires");
        assert!(r1cs.n_constraints > 0, "Should have constraints");
        assert_eq!(r1cs.constraints.len(), r1cs.n_constraints as usize);
        
        // Check first constraint structure
        if !r1cs.constraints.is_empty() {
            let c0 = &r1cs.constraints[0];
            println!("\nFirst constraint:");
            println!("  A factors: {}", c0.a.factors.len());
            println!("  B factors: {}", c0.b.factors.len());
            println!("  C factors: {}", c0.c.factors.len());
        }
    }
}
