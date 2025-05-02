use serde::Deserialize;

#[derive(Debug,Deserialize)]
struct ArrestRecord {
    ARREST_KEY: String,
    ARREST_DATE: String,
    PD_CD: Option<String>,
    PD_DESC: Option<String>,
    KY_CD: Option<String>,
    OFNS_DESC: Option<String>,
    LAW_CODE: Option<String>,
    LAW_CAT_CD: Option<String>,
    ARREST_BORO: Option<String>,
    ARREST_PRECINCT: Option<u32>,
    JURISDICTION_CODE: Option<u32>,
    AGE_GROUP: Option<String>,
    PERP_SEX: Option<String>,
    PERP_RACE: Option<String>,
    Latitude: Option<f64>,
    Longitude: Option<f64>,
}

fn read_csv(file: &str) -> Result<HashMap<u32,Vec<f32>>>, Box<dyn Error>> {
    
}