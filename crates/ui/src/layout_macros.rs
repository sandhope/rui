#[macro_export]
macro_rules! Col {
    { $( $label:expr )* } => {
        {
            let mut col = div().flex().flex_col().w_full();
            $(
                col = col.child($label);
            )*
            col
        }
    };
}

#[macro_export]
macro_rules! Row {
    { $( $label:expr )* } => {
        {
            let mut row = div().flex().flex_row().w_full();
            $(
                row = row.child($label);
            )*
            row
        }
    };
}
