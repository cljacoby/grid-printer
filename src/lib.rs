//! An API to easily print a two dimensional array to stdout.
//! # Example
//! ```rust
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
//! Output:
//! ```bash
//! Make           Model     Color     Year    Price
//! Ford           Pinto     Green     1978    $750.00
//! Toyota         Tacoma    Red       2006    15,475.23
//! Lamborghini    Diablo    Yellow    2001    $238,459.99
//! ```


use std::io;
use std::io::Write;
use std::fmt::Display;
use std::cell::RefCell;

/// An API to easily print a two dimensional array to stdout.
///
/// # Example
/// ```rust
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
/// Toyota         Tacoma    Red       2006    15,475.23
/// Lamborghini    Diablo    Yellow    2001    $238,459.99
/// ```
// #[derive(Debug)]
pub struct GridPrinter {
    rows: usize,
    cols: usize,
    buff: RefCell<Vec<String>>,
    max_widths: RefCell<Vec<usize>>,
    col_spacing: usize,
}

impl GridPrinter {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            cols,
            rows,
            ..GridPrinterBuilder::new(rows, cols).build()
        }
    }

    pub fn builder(rows: usize, cols: usize) -> GridPrinterBuilder {
        GridPrinterBuilder::new(rows, cols)
    }

    fn pad(n: usize) -> String {
        vec![' '; n].into_iter().collect()
    }

    pub fn print<F: Display>(&self, source: &Vec<Vec<F>>) {

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
                self.buff.borrow_mut().push(cell);
            }
        }


        let buff = self.buff.borrow();
        for (i, cell) in buff.iter().enumerate() {
            let col_width = self.max_widths.borrow()[i % self.cols];
            let pad = GridPrinter::pad(col_width - cell.len() + self.col_spacing);
            print!("{}{}", cell, pad);
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
}

impl Default for GridPrinterBuilder {
     fn default() -> Self {
        Self {
            rows: 1,
            cols: 1,
            col_spacing: 2,
        }
    }
}

impl GridPrinterBuilder {

    pub fn new(rows: usize, cols: usize) -> Self {
        let mut builder = GridPrinterBuilder::default(); 
        builder.rows = rows;
        builder.cols = cols;

        builder
    }

    pub fn col_spacing(mut self, col_spacing: usize) -> Self {
        self.col_spacing = col_spacing;

        self
    }


    pub fn build(self) -> GridPrinter {
        GridPrinter {
            rows: self.rows,
            cols: self.cols,
            buff: RefCell::new(Vec::with_capacity(self.rows * self.cols)),
            max_widths: RefCell::new(vec![0; self.cols]),
            col_spacing: self.col_spacing,
        }
    }

}


#[cfg(test)]
mod tests {

    use super::*;
    use rand::random;
    use std::time::Instant;


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

    fn create_test_grid(rows: usize, cols: usize) -> Vec<Vec<u8>> {
        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(rows);
        for i in 0..rows {
            grid.push(Vec::with_capacity(cols));
            let row = grid.get_mut(i).unwrap();
            for _j in 0..cols {
                row.push(random::<u8>());
            }
        }

        grid
    }

    // #[bench]
    #[test]
    fn bench_vs_vec() {
        let rows = 100;
        let cols = 100;
        let grid = create_test_grid(rows, cols);
        let printer = GridPrinterBuilder::new(rows, cols)
            .col_spacing(4)
            .build();
        
        let start = Instant::now();
        printer.print(&grid);
        let fin = Instant::now();
        let time_printer = fin.duration_since(start);
        println!("time = {:?}", time_printer);

        let start = Instant::now();
        for row in grid.iter() {
            for cell in row.iter() {
                print!("{}  ", cell);
            }
            print!("\n");
        }
        let fin = Instant::now();
        let time_printer = fin.duration_since(start);
        println!("time = {:?}", time_printer);

    }

}
