pub trait LaTeX {
    fn latex(&self) -> String;
}

// Store max width?
// pub trait ASCII {
//     fn ascii(&self) -> Vec<String>;

//     fn width(&self) -> usize;

//     fn height(&self) -> usize;
// }