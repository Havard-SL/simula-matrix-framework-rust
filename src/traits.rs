pub trait LaTeX {
    fn latex(&self) -> String;
}

pub trait SpreadsheetDisplay {
    fn spreadsheet_display(&self) -> String;
}

// Store max width?
// pub trait ASCII {
//     fn ascii(&self) -> Vec<String>;

//     fn width(&self) -> usize;

//     fn height(&self) -> usize;
// }
