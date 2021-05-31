//! An API to easily print a two dimensional array to stdout.
//! # Example
//! ```rust
//! use grid_printer::GridPrinter;
//!
//! let cars = vec![
//!     vec!["Make", "Model", "Color", "Year", "Price", ],
//!     vec!["Ford", "Pinto", "Green", "1978", "$750.00", ],
//!     vec!["Toyota", "Tacoma", "Red", "2006", "$15,475.23", ],
//!     vec!["Lamborghini", "Diablo", "Yellow", "2001", "$238,459.99", ],
//! ];
//! 
//! let rows = cars.len();
//! let cols = cars[0].len();
//! let printer = GridPrinter::builder(rows, cols)
//!     .col_spacing(4)
//!     .build();
//! printer.print(&cars);
//! ```
//!
//! # Output:
//! ```bash
//! Make           Model     Color     Year    Price
//! Ford           Pinto     Green     1978    $750.00
//! Toyota         Tacoma    Red       2006    $15,475.23
//! Lamborghini    Diablo    Yellow    2001    $238,459.99
//! ```

pub mod style;

use std::io;
use std::fmt;
use std::io::Write;
use std::fmt::Display;
use std::error::Error;
use std::cell::RefCell;

use crate::style::StyleOpt;
use crate::style::stylize;

/// An API to easily print a two dimensional array to stdout.
///
/// # Example
/// ```rust
/// use grid_printer::GridPrinter;
///
/// let cars = vec![
///     vec!["Make", "Model", "Color", "Year", "Price", ],
///     vec!["Ford", "Pinto", "Green", "1978", "$750.00", ],
///     vec!["Toyota", "Tacoma", "Red", "2006", "$15,475.23", ],
///     vec!["Lamborghini", "Diablo", "Yellow", "2001", "$238,459.99", ],
/// ];
/// 
/// let rows = cars.len();
/// let cols = cars[0].len();
/// let printer = GridPrinter::builder(rows, cols)
///     .col_spacing(4)
///     .build();
/// printer.print(&cars);
/// ```
///
/// Output:
/// ```bash
/// Make           Model     Color     Year    Price
/// Ford           Pinto     Green     1978    $750.00
/// Toyota         Tacoma    Red       2006    $15,475.23
/// Lamborghini    Diablo    Yellow    2001    $238,459.99
/// ```
// #[derive(Debug)]
pub struct GridPrinter {
    rows: usize,
    cols: usize,
    max_widths: RefCell<Vec<usize>>,
    col_spacing: usize,
    col_styles: Option<Vec<Option<StyleOpt>>>,
}

impl GridPrinter {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            ..GridPrinterBuilder::new(rows, cols).build()
        }
    }

    pub fn builder(rows: usize, cols: usize) -> GridPrinterBuilder {
        GridPrinterBuilder::new(rows, cols)
    }

    fn pad(n: usize) -> String {
        vec![' '; n].into_iter().collect()
    }

    #[allow(clippy::print_with_newline)]
    pub fn print_cell(&self, cell: &str, col_idx: usize, style_opt: Option<&StyleOpt>) {

        let mut s = cell.to_string(); 
        if let Some(style_opt) = style_opt {
            s = stylize(cell, style_opt);
        }
        let col_width = self.max_widths.borrow()[col_idx];
        let pad = GridPrinter::pad(col_width - cell.len() + self.col_spacing);
        print!("{}{}", s, pad);
    }

    #[allow(clippy::print_with_newline)]
    pub fn print<F: Display>(&self, source: &[Vec<F>]) {
        let mut buff: Vec<String> = Vec::new();

        for i in 0..self.rows {
            let row = source.get(i);
            for j in 0..self.cols {
                let cell = match row {
                    None => "".to_string(),
                    Some(row) => match row.get(j) {
                        None => "".to_string(),
                        Some(el) => format!("{}", el),
                    } 
                };
                let len = cell.len();
                if len > self.max_widths.borrow()[j] {
                    self.max_widths.borrow_mut()[j] = len;
                }
                // self.buff.borrow_mut().push(cell);
                buff.push(cell);
            }
        }


        for (i, cell) in buff.iter().enumerate() {
            let col_idx = i % self.cols;
            let _row_idx = i / self.rows;

            let style_opt = match self.col_styles.as_ref() {
                None => None,
                Some(col_styles) => match col_styles.get(col_idx) {
                    None => None,
                    Some(style_opt) => style_opt.as_ref(),
                }
            };

            self.print_cell(cell, col_idx, style_opt);

            if (i + 1) % self.cols == 0 {
                print!("\n");
                io::stdout().flush().unwrap();
            }
        }


    }
}

/// A Builder to create/customize a GridPrinter instance
/// ```rust
/// use grid_printer::GridPrinter;
/// use grid_printer::GridPrinterBuilder;
/// 
/// let rows = 3;
/// let cols = 3;
/// let printer: GridPrinter = GridPrinterBuilder::new(rows, cols)
///     .col_spacing(4)
///     .build();
/// ```
#[derive(Debug)]
pub struct GridPrinterBuilder {
    rows: usize,
    cols: usize,
    col_spacing: usize,
    col_styles: Option<Vec<Option<StyleOpt>>>,
}

impl Default for GridPrinterBuilder {
     fn default() -> Self {
        Self {
            rows: 1,
            cols: 1,
            col_spacing: 2,
            col_styles: None,
        }
    }
}

impl GridPrinterBuilder {

    pub fn new(rows: usize, cols: usize) -> Self {
        GridPrinterBuilder {
            rows,
            cols,
            ..Default::default()
        }
    }

    pub fn col_spacing(mut self, col_spacing: usize) -> Self {
        self.col_spacing = col_spacing;

        self
    }

    pub fn col_styles(mut self, col_styles: Vec<Option<StyleOpt>>) -> Result<Self, GridPrinterErr> {
        match col_styles.len() == self.cols {
            false => Err(GridPrinterErr::DimensionErr),
            true => {
                self.col_styles = Some(col_styles);

                Ok(self)
            }
        }
    }

    pub fn col_style(mut self, idx: usize, opt: StyleOpt) -> Result<Self, GridPrinterErr> {
        // Note: The size check here is somewhat redundant given the subsequent logic; however,
        // performing the check here guarantees we don't mutate the GridPrinterBuilder by adding
        // a Vec for an index that is outside the column range.
        if idx >= self.cols {
            return Err(GridPrinterErr::DimensionErr);
        }

        let col_styles = self.col_styles.get_or_insert(vec![None; self.cols]);
        let col_style = col_styles.get_mut(idx)
            .ok_or(GridPrinterErr::DimensionErr)?;
        *col_style = Some(opt);

        Ok(self)
    }

    pub fn build(self) -> GridPrinter {
        GridPrinter {
            rows: self.rows,
            cols: self.cols,
            max_widths: RefCell::new(vec![0; self.cols]),
            col_spacing: self.col_spacing,
            col_styles: self.col_styles,
        }
    }

}

#[derive(Debug)]
pub enum GridPrinterErr {
    DimensionErr,
}

impl Display for GridPrinterErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GridPrinterErr::DimensionErr => {
                write!(f, "DimensionErr. Caused by mismatch in dimension size between method calls.")
            },
        }
    }
}

impl Error for GridPrinterErr {}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_2d_arr() {
        let v = vec![
            vec![1, 20, 3, ],
            vec![40, 5, 6, ],
            vec![7, 800, 9, ],
        ];

        let rows = v.len();
        let cols = v[0].len();
        let printer = GridPrinterBuilder::new(rows, cols)
            .col_spacing(20)
            .build();
        printer.print(&v);
    }

}
