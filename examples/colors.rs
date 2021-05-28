use grid_printer::GridPrinter;
use grid_printer::style::StyleOpt;
use grid_printer::style::FgColor;
use grid_printer::style::BgColor;

fn main() {

    let grid = vec![
        vec![1, 2, 3, 4, ],
        vec![5, 6, 7, 8, ],
        vec![9, 10, 11, 12, ],
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    let mut col_colors: Vec<Option<StyleOpt>> = vec![None; cols];
    col_colors[2] = Some(StyleOpt { fg: Some(FgColor::Red), bg: None });
    col_colors[3] = Some(StyleOpt { fg: Some(FgColor::Cyan), bg: None });

    let printer = GridPrinter::builder(rows, cols)
        .col_spacing(4)
        .col_colors(col_colors)
        .build();
    printer.print(&grid);
}
