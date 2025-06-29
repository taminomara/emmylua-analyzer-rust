use std::path::PathBuf;

use emmylua_code_analysis::{DbIndex, FileId, LuaDocument};
use lsp_types::{Diagnostic, DiagnosticSeverity};

#[derive(Debug)]
pub struct TerminalDisplay {
    workspace: PathBuf,
    supports_color: bool,
}

impl TerminalDisplay {
    pub fn new(workspace: PathBuf) -> Self {
        let supports_color = atty::is(atty::Stream::Stdout);

        Self {
            workspace,
            supports_color,
        }
    }

    pub fn display_diagnostics(
        &mut self,
        db: &DbIndex,
        file_id: FileId,
        diagnostics: Vec<Diagnostic>,
    ) {
        if diagnostics.is_empty() {
            return;
        }

        let file_path = self.get_relative_path(db, file_id);
        let document = db.get_vfs().get_document(&file_id).unwrap();
        let text = document.get_text();
        let text_lines = text.lines().collect::<Vec<&str>>();

        // Group statistics by severity level
        let mut error_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;
        let mut hint_count = 0;

        for diagnostic in &diagnostics {
            match diagnostic.severity {
                Some(DiagnosticSeverity::ERROR) => error_count += 1,
                Some(DiagnosticSeverity::WARNING) => warning_count += 1,
                Some(DiagnosticSeverity::INFORMATION) => info_count += 1,
                Some(DiagnosticSeverity::HINT) => hint_count += 1,
                _ => {}
            }
        }

        // Print file header information
        self.print_file_header(
            &file_path,
            error_count,
            warning_count,
            info_count,
            hint_count,
        );

        // Display each diagnostic individually
        for diagnostic in diagnostics {
            self.display_single_diagnostic(&file_path, &document, &text_lines, diagnostic);
        }

        println!(); // Add blank line separator
    }

    fn get_relative_path(&self, db: &DbIndex, file_id: FileId) -> String {
        let mut file_path = db.get_vfs().get_file_path(&file_id).unwrap().clone();
        if let Ok(new_file_path) = file_path.strip_prefix(&self.workspace) {
            file_path = new_file_path.to_path_buf();
        }
        file_path.to_string_lossy().to_string()
    }

    fn print_file_header(
        &self,
        file_path: &str,
        error_count: usize,
        warning_count: usize,
        info_count: usize,
        hint_count: usize,
    ) {
        if self.supports_color {
            print!("\x1b[1;36m"); // Bright cyan, bold
        }
        print!("--- ");
        if self.supports_color {
            print!("\x1b[1;37m"); // Bright white, bold
        }
        print!("{}", file_path);
        if self.supports_color {
            print!("\x1b[1;36m"); // Bright cyan, bold
        }
        print!(" ");

        let mut parts = Vec::new();
        if error_count > 0 {
            if self.supports_color {
                parts.push(format!(
                    "\x1b[1;31m{} error{}\x1b[0m",
                    error_count,
                    if error_count > 1 { "s" } else { "" }
                ));
            } else {
                parts.push(format!(
                    "{} error{}",
                    error_count,
                    if error_count > 1 { "s" } else { "" }
                ));
            }
        }
        if warning_count > 0 {
            if self.supports_color {
                parts.push(format!(
                    "\x1b[1;33m{} warning{}\x1b[0m",
                    warning_count,
                    if warning_count > 1 { "s" } else { "" }
                ));
            } else {
                parts.push(format!(
                    "{} warning{}",
                    warning_count,
                    if warning_count > 1 { "s" } else { "" }
                ));
            }
        }
        if info_count > 0 {
            if self.supports_color {
                parts.push(format!("\x1b[1;34m{} info\x1b[0m", info_count));
            } else {
                parts.push(format!("{} info", info_count));
            }
        }
        if hint_count > 0 {
            if self.supports_color {
                parts.push(format!(
                    "\x1b[1;35m{} hint{}\x1b[0m",
                    hint_count,
                    if hint_count > 1 { "s" } else { "" }
                ));
            } else {
                parts.push(format!(
                    "{} hint{}",
                    hint_count,
                    if hint_count > 1 { "s" } else { "" }
                ));
            }
        }

        if !parts.is_empty() {
            print!("[{}]", parts.join(", "));
        }

        if self.supports_color {
            print!("\x1b[0m"); // Reset colors
        }
        println!();
    }

    fn display_single_diagnostic(
        &mut self,
        file_path: &str,
        document: &LuaDocument,
        lines: &[&str],
        diagnostic: Diagnostic,
    ) {
        let range = diagnostic.range;
        // Get severity level colors and symbols
        let (level_color, level_symbol, _level_name) = match diagnostic.severity {
            Some(DiagnosticSeverity::ERROR) => ("\x1b[1;31m", "error", "error"),
            Some(DiagnosticSeverity::WARNING) => ("\x1b[1;33m", "warning", "warning"),
            Some(DiagnosticSeverity::INFORMATION) => ("\x1b[1;34m", "info", "info"),
            Some(DiagnosticSeverity::HINT) => ("\x1b[1;35m", "hint", "hint"),
            _ => ("\x1b[1;31m", "error", "error"),
        };

        // Get diagnostic code
        let code = match diagnostic.code {
            Some(code) => match code {
                lsp_types::NumberOrString::Number(code) => format!("[{}]", code),
                lsp_types::NumberOrString::String(code) => format!("[{}]", code),
            },
            _ => String::new(),
        };

        // Calculate line and column numbers
        let start_line = range.start.line as usize;
        let start_character = range.start.character as usize;
        let Some(start_col) = document.get_col_offset_at_line(start_line, start_character) else {
            return;
        };
        let start_col = u32::from(start_col) as usize;
        let end_line = range.end.line as usize;
        let end_character = range.end.character as usize;
        let Some(end_col) = document.get_col_offset_at_line(end_line, end_character) else {
            return;
        };
        let end_col = u32::from(end_col) as usize;

        if start_line >= lines.len() {
            return;
        }

        // Print diagnostic header
        if self.supports_color {
            print!("{}{}:\x1b[0m ", level_color, level_symbol);
            print!("\x1b[1m{}\x1b[0m", diagnostic.message);
            if !code.is_empty() {
                print!(" \x1b[90m{}\x1b[0m", code);
            }
        } else {
            print!("{}: {}", level_symbol, diagnostic.message);
            if !code.is_empty() {
                print!(" {}", code);
            }
        }
        println!();

        // Print location information
        if self.supports_color {
            println!(
                "  \x1b[90m-->\x1b[0m {}:{}:{}",
                file_path,
                start_line + 1,
                start_character + 1
            );
        } else {
            println!(
                "  --> {}:{}:{}",
                file_path,
                start_line + 1,
                start_character + 1
            );
        }

        // Calculate context range to display (one line before and after for context)
        let context_start = if start_line > 0 { start_line - 1 } else { 0 };
        let context_end = std::cmp::min(end_line + 1, lines.len() - 1);

        // Calculate maximum line number width for alignment
        let max_line_num = context_end + 1;
        let line_num_width = max_line_num.to_string().len();

        println!(); // Empty line separator

        // Display code lines
        for (i, line_text) in lines
            .iter()
            .enumerate()
            .take(context_end + 1)
            .skip(context_start)
        {
            let line_num = i + 1;

            if self.supports_color {
                print!(
                    "  \x1b[94m{:width$}\x1b[0m │ ",
                    line_num,
                    width = line_num_width
                );
            } else {
                print!("  {:width$} | ", line_num, width = line_num_width);
            }

            if i >= start_line && i <= end_line {
                // This is an error line, needs highlighting
                if i == start_line && i == end_line {
                    // Single line error
                    let prefix = &line_text[..std::cmp::min(start_col, line_text.len())];
                    let error_part = if start_col < line_text.len() && end_col <= line_text.len() {
                        &line_text[start_col..end_col]
                    } else if start_col < line_text.len() {
                        &line_text[start_col..]
                    } else {
                        ""
                    };
                    let suffix = if end_col < line_text.len() {
                        &line_text[end_col..]
                    } else {
                        ""
                    };

                    print!("{}", prefix);
                    if self.supports_color && !error_part.is_empty() {
                        print!("{}{}\x1b[0m", level_color, error_part);
                    } else {
                        print!("{}", error_part);
                    }
                    println!("{}", suffix);
                } else {
                    // Start or end line of multi-line error
                    if self.supports_color {
                        println!("{}{}\x1b[0m", level_color, line_text);
                    } else {
                        println!("{}", line_text);
                    }
                }
            } else {
                // Context line
                println!("{}", line_text);
            }
        }

        println!(); // Ending empty line
    }

    pub fn print_summary(
        &self,
        total_errors: usize,
        total_warnings: usize,
        total_info: usize,
        total_hints: usize,
    ) {
        if total_errors == 0 && total_warnings == 0 && total_info == 0 && total_hints == 0 {
            if self.supports_color {
                println!("\x1b[1;32mNo issues found\x1b[0m");
            } else {
                println!("No issues found");
            }
            return;
        }

        println!();
        if self.supports_color {
            print!("\x1b[1;36m"); // Bright cyan, bold
        }
        print!("Summary");
        if self.supports_color {
            print!("\x1b[0m"); // Reset colors
        }
        println!();

        if total_errors > 0 {
            if self.supports_color {
                println!(
                    "  \x1b[1;31m{} error{}\x1b[0m",
                    total_errors,
                    if total_errors > 1 { "s" } else { "" }
                );
            } else {
                println!(
                    "  {} error{}",
                    total_errors,
                    if total_errors > 1 { "s" } else { "" }
                );
            }
        }

        if total_warnings > 0 {
            if self.supports_color {
                println!(
                    "  \x1b[1;33m{} warning{}\x1b[0m",
                    total_warnings,
                    if total_warnings > 1 { "s" } else { "" }
                );
            } else {
                println!(
                    "  {} warning{}",
                    total_warnings,
                    if total_warnings > 1 { "s" } else { "" }
                );
            }
        }

        if total_info > 0 {
            if self.supports_color {
                println!("  \x1b[1;34m{} info\x1b[0m", total_info);
            } else {
                println!("  {} info", total_info);
            }
        }

        if total_hints > 0 {
            if self.supports_color {
                println!(
                    "  \x1b[1;35m{} hint{}\x1b[0m",
                    total_hints,
                    if total_hints > 1 { "s" } else { "" }
                );
            } else {
                println!(
                    "  {} hint{}",
                    total_hints,
                    if total_hints > 1 { "s" } else { "" }
                );
            }
        }

        // Final status
        if total_errors > 0 {
            if self.supports_color {
                println!("\n\x1b[1;31mCheck failed\x1b[0m");
            } else {
                println!("\nCheck failed");
            }
        } else if total_warnings > 0 {
            if self.supports_color {
                println!("\n\x1b[1;33mCheck completed with warnings\x1b[0m");
            } else {
                println!("\nCheck completed with warnings");
            }
        } else {
            if self.supports_color {
                println!("\n\x1b[1;32mCheck successful\x1b[0m");
            } else {
                println!("\nCheck successful");
            }
        }
    }
}
