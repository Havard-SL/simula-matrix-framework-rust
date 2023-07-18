use spreadsheet_ods::{WorkBook, Sheet, Value};
use spreadsheet_ods::style::CellStyle;
use color::Rgb;
use icu_locid::locale;

use spreadsheet_ods::ValueFormatText;

use crate::structs::{Table, SpreadsheetColours};
use crate::structs::traits::SpreadsheetDisplay;


pub fn write_table_to_spreadsheet<L, R>(
    table: &Table<L, R>,
) -> Result<(), spreadsheet_ods::OdsError>
where
    L: SpreadsheetDisplay,
    R: SpreadsheetDisplay,
{
    // let path = std::path::Path::new("test/example.ods");
    // let mut wb = if path.exists() {
    //     spreadsheet_ods::read_ods(path).unwrap()
    // } else {
    //     WorkBook::new(locale!("en_US"))
    // };

    let mut wb = WorkBook::new(locale!("en_US"));

    if wb.num_sheets() == 0 {
        let mut sheet = Sheet::new("test");
        sheet.set_value(0, 0, true);
        wb.push_sheet(sheet);
    }

    let color_style_automorphism = ValueFormatText::new_named("Automorphism 1");
    let color_style_automorphism = wb.add_text_format(color_style_automorphism);
    let mut color_style_automorphism = CellStyle::new("Automorphism 2", &color_style_automorphism);
    color_style_automorphism.set_background_color(Rgb::new(77, 166, 255));
    let color_style_automorphism = wb.add_cellstyle(color_style_automorphism);


    let color_style_affine_automorphism = ValueFormatText::new_named("Affine Automorphism 1");
    let color_style_affine_automorphism = wb.add_text_format(color_style_affine_automorphism);
    let mut color_style_affine_automorphism = CellStyle::new("Affine Automorphism 2", &color_style_affine_automorphism);
    color_style_affine_automorphism.set_background_color(Rgb::new(255, 255, 102));
    let color_style_affine_automorphism = wb.add_cellstyle(color_style_affine_automorphism);

    let color_style_automorphism_and_affine_automorphism = ValueFormatText::new_named("Automorphism And Affine Automorphism 1");
    let color_style_automorphism_and_affine_automorphism = wb.add_text_format(color_style_automorphism_and_affine_automorphism);
    let mut color_style_automorphism_and_affine_automorphism = CellStyle::new("Automorphism And Affine Automorphism 2", &color_style_automorphism_and_affine_automorphism);
    color_style_automorphism_and_affine_automorphism.set_background_color(Rgb::new(85, 255, 51));
    let color_style_automorphism_and_affine_automorphism = wb.add_cellstyle(color_style_automorphism_and_affine_automorphism);

    let sheet = wb.sheet_mut(0);

    for (i, rows) in table.left.iter().enumerate() {
        let n = rows.len();
        
        for (j, v) in rows.iter().enumerate() {
            sheet.set_value(
                i.try_into().unwrap(),
                j.try_into().unwrap(),
                Value::Text(v.spreadsheet_display()),
            )
        }

        for (j, v) in table.right[i].iter().enumerate() {
            
            match v.color() {
                SpreadsheetColours::Automorphism(_) => {
                    sheet.set_styled_value(
                        i.try_into().unwrap(),
                        (j + n).try_into().unwrap(),
                        Value::Text(v.spreadsheet_display()),
                        &color_style_automorphism,
                    ) 
                }
                SpreadsheetColours::AffineAutomorphism(_) => {
                    sheet.set_styled_value(
                        i.try_into().unwrap(),
                        (j + n).try_into().unwrap(),
                        Value::Text(v.spreadsheet_display()),
                        &color_style_affine_automorphism,
                    ) 
                }
                SpreadsheetColours::AutomorphismAndAffine(_) => {
                    sheet.set_styled_value(
                        i.try_into().unwrap(),
                        (j + n).try_into().unwrap(),
                        Value::Text(v.spreadsheet_display()),
                        &color_style_automorphism_and_affine_automorphism,
                    ) 
                }
                SpreadsheetColours::NoColor => {
                    sheet.set_value(
                        i.try_into().unwrap(),
                        (j + n).try_into().unwrap(),
                        Value::Text(v.spreadsheet_display()),
                    )
                }
            }
        }
    }

    spreadsheet_ods::write_ods(&mut wb, "test/example.ods")
}
