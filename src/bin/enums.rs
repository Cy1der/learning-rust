fn main() {
    // An enum in Rust is a type that represents data that could be one of several possible variants

    // Basic example

    enum Coordinates {
        Location { x: i32, y: i32 },
    }

    /*
    Each variant can optionally have data associated with it. The syntax for defining variants resembles the syntaxes used to define structs: you can have variants with no data (like unit-like structs), variants with named data, and variants with unnamed data (like tuple structs). Unlike separate struct definitions, however, an enum is a single type. A value of the enum can match any of the variants. For this reason, an enum is sometimes called a ‘sum type’: the set of possible values of the enum is the sum of the sets of possible values for each variant.
    */

    // We use the :: syntax to use the name of each variant: they’re scoped by the name of the enum itself
    // This allows both of these to work

    let coords: Coordinates = Coordinates::Location { x: 10, y: 30 };

    enum Coordinates2 {
        Location { x: i32, y: i32 },
    }

    let coords_2: Coordinates2 = Coordinates2::Location { x: 10, y: 30 };
}
