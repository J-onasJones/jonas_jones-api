use ip2location::{DB, Record};

pub fn ip_lookup(ip: &str) -> String {
    let database_path = "resources/IP2LOCATION-LITE-DB5.IPV6.BIN/IP2LOCATION-LITE-DB5.IPV6.BIN";

    let mut db = DB::from_file(database_path).unwrap();

    let geo_info = db.ip_lookup(ip.parse().unwrap()).unwrap();

    let record = if let Record::LocationDb(rec) = geo_info {
        Some(rec)
    } else { None };
    return record.unwrap().country.unwrap().short_name.to_string();
}