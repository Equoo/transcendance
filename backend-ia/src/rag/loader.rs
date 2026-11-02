use anyhow::{Context, Result, anyhow};
use docx_lite::{extract_text_from_bytes};
use calamine::{Data, Reader, open_workbook_auto_from_rs};
use std::io::Cursor;

pub trait DocumentLoader {
    fn load(&self, bytes: &[u8]) -> Result<String>;
}

struct PlainTextLoader;
impl DocumentLoader for PlainTextLoader {
    fn load(&self, bytes: &[u8]) -> Result<String> {
        let content = std::str::from_utf8(bytes)
            .with_context(|| "Unable to read the file content as text")?;
        Ok(content.to_string())
    }
}

struct PdfLoader;
impl DocumentLoader for PdfLoader {
    fn load(&self, bytes: &[u8]) -> Result<String> {
        let content = pdf_extract::extract_text_from_mem(bytes)
            .with_context(|| "Unable to extract text from the PDF")?;
        Ok(content)
    }
}

struct DocxLoader;
impl  DocumentLoader for DocxLoader{
	fn load(&self, bytes: &[u8]) -> Result<String> {
		let content = extract_text_from_bytes(bytes)
			.with_context(|| "Unable to extract text from the DOCX")?;
		Ok(content)
	}
}

struct SpreadsheetLoader;
impl DocumentLoader for SpreadsheetLoader {
	fn load(&self, bytes: &[u8]) -> Result<String> {
		let mut workbook = open_workbook_auto_from_rs(Cursor::new(bytes)).with_context(|| "Unable to extract text from doc")?;
		
		let mut content = String::new();
		
		for name in workbook.sheet_names()
		{
			let result = workbook.worksheet_range(&name).with_context(|| format!("Unable to read the sheet {name}"))?;

			let mut rows = result.rows();
			let Some(header_row) = rows.next()
			else 
			{
				continue;
			};

			let headers: Vec<String> = header_row.iter().map(|c| c.to_string()).collect();

			content.push_str(&format!("## Sheet : {name}\n"));
			for row in rows {
				let line: Vec<String> = row
					.iter()
					.zip(&headers)
					.filter(|(cell, _)| !matches!(cell, Data::Empty))
					.map(|(cell, header): (&Data, &String)| {
						if header.is_empty() {
							cell.to_string()
						} else {
							format!("{header}: {cell}")
						}
					})
					.collect();

				if !line.is_empty() {
					content.push_str(&line.join(" | "));
					content.push('\n');
				}
			}
			content.push('\n');
			
		}
		Ok(content)
	}
}

pub fn load_document(bytes: &[u8], extension: &str) -> Result<String> {
    let loader: Box<dyn DocumentLoader> = match extension.to_lowercase().as_str() {
        "txt" | "md" | "csv" | "json" | "log" | "xml" | "yaml" => Box::new(PlainTextLoader),
        "pdf" => Box::new(PdfLoader),
		"docx" => Box::new(DocxLoader),
		"xlsx" | "xls" | "xlsm" | "ods" => Box::new(SpreadsheetLoader),
        other => return Err(anyhow!("Unsuported Format : .{other}")),
    };

    loader.load(bytes)
}

pub fn chunk_text(text: &str, max_chars: usize) -> Vec<String> {
    text.chars()
        .collect::<Vec<char>>()
        .chunks(max_chars)
        .map(|c| c.iter().collect())
        .collect()
}
