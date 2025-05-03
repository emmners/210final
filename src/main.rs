mod csv;
use crate::csv::read_csv;

fn main() {
    let path = "updated_NYPD_Arrest_Data_YTD.csv";
    let records = read_csv(path);
    println!("{:?}", records);
    println!("Hello World");
}
