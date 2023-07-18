use std::fmt::Display;

use icu_locid::locale;
use spreadsheet_ods::{Sheet, Value, WorkBook};

use crate::table::Table;

pub fn write_table_to_spreadsheet<L, R>(
    table: &Table<L, R>,
) -> Result<(), spreadsheet_ods::OdsError>
where
    L: Display,
    R: Display,
{
    let path = std::path::Path::new("test/example.ods");
    let mut wb = if path.exists() {
        spreadsheet_ods::read_ods(path).unwrap()
    } else {
        WorkBook::new(locale!("en_US"))
    };

    if wb.num_sheets() == 0 {
        let mut sheet = Sheet::new("test");
        sheet.set_value(0, 0, true);
        wb.push_sheet(sheet);
    }

    let sheet = wb.sheet_mut(0);

    for (i, rows) in table.left.iter().enumerate() {
        let n = rows.len();

        for (j, v) in rows.iter().enumerate() {
            sheet.set_value(
                i.try_into().unwrap(),
                j.try_into().unwrap(),
                Value::Text(v.to_string()),
            )
        }

        for (j, v) in table.right[i].iter().enumerate() {
            sheet.set_value(
                i.try_into().unwrap(),
                (j + n).try_into().unwrap(),
                Value::Text(v.to_string()),
            )
        }
    }

    spreadsheet_ods::write_ods(&mut wb, "test/example.ods")
}
