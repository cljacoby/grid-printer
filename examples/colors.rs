use grid_printer::GridPrinter;
use grid_printer::style::StyleOpt;
use grid_printer::style::Fg;
use grid_printer::style::Bg;
use grid_printer::style::Style;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let grid = vec![
        vec![1, 2, 3, 4, ],
        vec![5, 6, 7, 8, ],
        vec![9, 10, 11, 12, ],
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    
    let printer = GridPrinter::builder(rows, cols)
        .col_style(0, StyleOpt::new().fg(Fg::Magenta))?
        .col_style(1, StyleOpt::new().fg(Fg::Black).bg(Bg::BrightYellow))?
        .col_style(2, StyleOpt::new().style(Style::StrikeThrough))?
        .col_style(3, StyleOpt::new()
            .fg(Fg::Black)
            .bg(Bg::White)
            .style(Style::Italic))?
        .build();
    
    printer.print(&grid);

    Ok(())
}

