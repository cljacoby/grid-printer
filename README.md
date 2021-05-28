# grid-printer
An API to easily print a two dimensional array to stdout.

##### Example
```rust
let cars = vec![
    vec!["Make", "Model", "Color", "Year", "Price", ],
    vec!["Ford", "Pinto", "Green", "1978", "$750.00", ],
    vec!["Toyota", "Tacoma", "Red", "2006", "$15,475.23", ],
    vec!["Lamborghini", "Diablo", "Yellow", "2001", "$238,459.99", ],
];

let rows = cars.len();
let cols = cars[0].len();
let printer = GridPrinter::builder(rows, cols)
    .col_spacing(4)
    .build();
printer.print(&cars);
```

##### Output
```bash
Make           Model     Color     Year    Price
Ford           Pinto     Green     1978    $750.00
Toyota         Tacoma    Red       2006    15,475.23
Lamborghini    Diablo    Yellow    2001    $238,459.99
```

