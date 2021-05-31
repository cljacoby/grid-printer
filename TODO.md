### Features to Add
- [ ] Row stylization. Consider how conflicting/overlapping column and
    row stylization is resolved.

### Possible Features
- [ ] Style based on closure. Allow passing a closure with respect to
    either the rows or columns axis, add conditionally stylize the
    axis. Possible ergonomic outline:
    ```rust
    // Conditionally stylize a column as either Red or Green based on
    // whether the cell value is greater than 0
    let printer = GridPrinter::builder(3, 3)
        .style_func(
            Axis::Col,
            StyleOpt::new().fg(Fg::Green),
            StyleOpt::new().bg(Bg::Red),
            |cell: i32| cell > 0
        )?
        .build();
    printer.print(&cars);
    ```
- [ ] Use of const generics (or another suitable mechanism) to allow the
    builder to resolve all argument setting at compile time. Currently,
    when setting a column style, it always possible that the caller sets
    a column index outside the ranger of the printer. Currently this has to
    resolve at runtime with a `Result`:
    ```rust
    // The call to `col_style()` passes and index outside the range of
    // GridPrinter. It would be nice if this was a compile time
    // mechanism using the type system, as this would allow the entire
    // crate to never return `Result`s
    let printer = GridPrinter::builder(3, 3)
        .col_style(3, StyleOpt::new().fg(Fg::Red))?
        .build();
    printer.print(&cars);

    ```
