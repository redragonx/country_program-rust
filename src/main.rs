extern crate csv;
extern crate serialize;

use std::io::File;
use std::io::BufferedReader;
use std::path::Path;
use std::fmt;

const MIN_YEAR: uint = 1986;
const MAX_YEAR: uint = 2006;
const MIN_GEO_AREA_NUMBER: uint = 1;
const MAX_GEO_AREA_NUMBER: uint = 12;

#[deriving(Decodable, Clone)]
struct CountryRecord {
    name: String,
    exports: uint,
    trade_balance: int,
    year: uint,
    population: uint,
    geo_area: uint,
}

#[deriving(Decodable, Clone, Default)]
struct RatioRecord {
    country_name: String,
    country_ratio: int,
}

impl CountryRecord {
    fn cmp_by_exports(&self, other: &CountryRecord) -> Ordering {
        self.exports.cmp(&other.exports)
    }

    fn cmp_by_trade_balance(&self, other: &CountryRecord) -> Ordering {
        self.trade_balance.cmp(&other.trade_balance)
    }
}

impl RatioRecord {
    fn cmp_by_best_export_to_tb_ratio(&self, other: &RatioRecord) -> Ordering {
        self.country_ratio.cmp(&other.country_ratio)
    }
}

impl fmt::Show for CountryRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}, {}, {}, {}",
               self.name,
               self.exports,
               self.trade_balance,
               self.year,
               self.population,
               self.geo_area)
    }
}

impl fmt::Show for RatioRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}",
               self.country_name,
               self.country_ratio)
    }
}

fn main() {
    let filepath = get_file_name();

    // Strings can have null bytes
    let master_country_list = load_file_into_vector(filepath.as_slice().trim_right());
    let mut user_option;

    loop {
        display_menu();

        user_option = get_user_choice();
        if user_option == None {
            println!{"That was not a valid option."};
            continue;
        }

        // get the actual value.
        match user_option.unwrap() {
            1 => {
                let top5 = get_top5_export_countries(master_country_list.clone());
                let mut i = 1u;

                println!("------------------------TOP 5 EXPORTING COUNTRIES-------------------------");
                for top_country in top5.iter() {
                    println!("{}. {}", i, top_country);
                    i = i + 1u;
                }
                println!("--------------------------------------------------------------------------");
                println!("");
            },
            2 => {
                let worst5 = get_top5_worst_export_countries(master_country_list.clone());
                let mut i = 1u;

                println!("------------------------TOP 5 WORST EXPORTING COUNTRIES-------------------");
                for worst_country in worst5.iter() {
                    println!("{}. {}", i, worst_country);
                    i = i + 1u;
                }
                println!("--------------------------------------------------------------------------");
                println!("");
            },
            3 => { // top 5 tb countries
                let top5_trade_balance = get_top5_trade_balance_countries(master_country_list.clone());
                let mut i = 1u;

                println!("------------------------TOP 5 TRADE BALANCE EXPORTING COUNTRIES-----------");
                for top_tb_country in top5_trade_balance.iter() {
                    println!("{}. {}", i, top_tb_country);
                    i = i + 1u;
                }
                println!("--------------------------------------------------------------------------");
                println!("");
            },
            4 => {
                let worst5_trade_balance = get_worst5_trade_balance_countries(master_country_list.clone());
                let mut i = 1u;

                println!("------------------------TOP 5 WORST TRADE BALANCE EXPORTING COUNTRIES-----------");
                for worst_tb_country in worst5_trade_balance.iter() {
                    println!("{}. {}", i, worst_tb_country);
                    i = i + 1u;
                }
                println!("--------------------------------------------------------------------------");
                println!("");
            },
            5 => {
                let top5_export_to_ratio_countries = get_best5_ratio_export_to_tb(master_country_list.clone());
                let mut i = 1u;

                println!("------------------------TOP 5 BEST EXPORT TO TB COUNTRIES-----------------");
                for best_export_to_ratio_country in top5_export_to_ratio_countries.iter() {
                    println!("{}. {}", i, best_export_to_ratio_country);
                    i = i + 1u;
                }
                println!("--------------------------------------------------------------------------");
                println!("");
            },
            6 => {
                let top5_worse_export_to_ratio_countries = get_worst5_ratio_export_to_tb(master_country_list.clone());

                let mut i = 1u;

                println!("------------------------TOP 5 WORSE EXPORT TO TB COUNTRIES----------------");
                for worst_export_to_ratio_country in top5_worse_export_to_ratio_countries.iter() {
                    println!("{}. {}", i, worst_export_to_ratio_country);
                    i = i + 1u;
                }
                println!("--------------------------------------------------------------------------");
                println!("");
            },
            7 => {
                let top5_geo_area_export_countries = get_top5_geo_area_exporting_countries(master_country_list.clone());

                let mut i = 1u;

                println!("------------------------TOP 5 GEO AREA COUNTRY RECORDS----------------");
                for best_geo_area_export_country in top5_geo_area_export_countries.iter() {
                    println!("{}. {}", i, best_geo_area_export_country);
                    i = i + 1u;
                }
                println!("--------------------------------------------------------------------------");
                println!("");
            },
            8 => break,
            _ => continue,
        }
    }
}

fn display_menu() {
    let menu = "1. Top 5 exporting countries \n\
                2. Top 5 worse exporting countries \n\
                3. Find 5 countries with the best trade balance \n\
                4. Find 5 countries with the worse trade balance \n\
                5. Find the best ratio of exports to TB \n\
                6. Find the worse ratio of exports to TB \n\
                7. Find the top 5 exporting country records in a geo-area \n\
                8. Quit \n";

    println!("Choose an option to execute.");
    println!("\n -----------------------------------");
    println!("{}", menu);
}


fn get_user_choice() -> Option<uint> {
    let user_num: Option<uint> = from_str(get_user_raw_input().as_slice().trim());

    user_num
}

fn get_top5_geo_area_exporting_countries(country_record_vector: Vec<CountryRecord>) -> Vec<CountryRecord> {
    let mut top5_geo_area_records = vec![];
    let user_geo_area_number = get_geo_area_from_user();

    for country in country_record_vector.into_iter() {
        if country.geo_area == user_geo_area_number{
            top5_geo_area_records.push(country);
        }
    }

    top5_geo_area_records.sort_by(|a, b| a.cmp_by_exports(b));
    top5_geo_area_records.reverse();

    top5_geo_area_records.into_iter().take(5).collect()
}

fn get_best5_ratio_export_to_tb(country_record_vector: Vec<CountryRecord>) -> Vec<RatioRecord>{
    let mut top5_ratio_records = vec![];
    let user_year = get_year();

    for country in country_record_vector.into_iter() {
        if country.year == user_year {
            let ratio = country.exports as int / country.trade_balance;

            let new_ratio_record = RatioRecord{ country_name: country.name,
                                                country_ratio: ratio};

            top5_ratio_records.push(new_ratio_record);
        }
    }

    top5_ratio_records.sort_by(|a, b| a.cmp_by_best_export_to_tb_ratio(b));
    top5_ratio_records.reverse();

    // best export to tb countries 
    top5_ratio_records.into_iter().take(5).collect()
}

fn get_worst5_ratio_export_to_tb(country_record_vector: Vec<CountryRecord>) -> Vec<RatioRecord>{
    let mut top5_ratio_records = vec![];
    let user_year = get_year();

    for country in country_record_vector.into_iter() {
        if country.year == user_year {
            let ratio = country.exports as int / country.trade_balance;

            let new_ratio_record = RatioRecord{ country_name: country.name,
                                                country_ratio: ratio};

            top5_ratio_records.push(new_ratio_record);
        }
    }

    top5_ratio_records.sort_by(|a, b| a.cmp_by_best_export_to_tb_ratio(b));

    // best export to tb countries 
    top5_ratio_records.into_iter().take(5).collect()
}

fn get_top5_export_countries(country_record_vector: Vec<CountryRecord>) -> Vec<CountryRecord> {
    let mut top5_countries: Vec<CountryRecord> = vec![];

    let user_year = get_year();

    // grab just the records that has user_year 
    for country in country_record_vector.into_iter() {

        if country.year == user_year {
            top5_countries.push(country)
        }
    }

    top5_countries.sort_by(|a, b| a.cmp_by_exports(b));
    top5_countries.reverse();

    // top 5 exporting countries
    top5_countries.into_iter().take(5).collect()
}

fn get_top5_trade_balance_countries(country_record_vector: Vec<CountryRecord>) -> Vec<CountryRecord> {
    let mut top5_countries: Vec<CountryRecord> = vec![];

    let user_year = get_year();

    // grab just the records that has user_year 
    for country in country_record_vector.into_iter() {

        if country.year == user_year {
            top5_countries.push(country)
        }
    }

    top5_countries.sort_by(|a, b| a.cmp_by_trade_balance(b));
    top5_countries.reverse();

    // best trade countries
    top5_countries.into_iter().take(5).collect()
}

fn get_worst5_trade_balance_countries(country_record_vector: Vec<CountryRecord>) -> Vec<CountryRecord> {
    let mut top5_countries: Vec<CountryRecord> = vec![];

    let user_year = get_year();

    // grab just the records that has user_year 
    for country in country_record_vector.into_iter() {

        if country.year == user_year {
            top5_countries.push(country)
        }
    }

    top5_countries.sort_by(|a, b| a.cmp_by_trade_balance(b));

    // worst 5 trade countries
    top5_countries.into_iter().take(5).collect()
}

fn get_top5_worst_export_countries(country_record_vector: Vec<CountryRecord>) -> Vec<CountryRecord> {
    let mut top5_countries: Vec<CountryRecord> = vec![];

    let user_year = get_year();

    // grab just the records that has user_year 
    for country in country_record_vector.into_iter() {

        if country.year == user_year {
            top5_countries.push(country)
        }
    }

    top5_countries.sort_by(|a, b| a.cmp_by_exports(b));

    // bottom five export countries
    top5_countries.into_iter().take(5).collect()
}

fn get_file_name() -> String {
    println!("Enter the file path to the data file.");
    get_user_raw_input()
}

fn get_user_raw_input() -> String {
    std::io::stdin().read_line().ok().expect("Failed to read line")
}

fn load_file_into_vector(file_path_str: &str) -> Vec<CountryRecord> {
    println!("loading: {}", file_path_str);

    let path = Path::new(file_path_str);
    let file_display = path.display();

    // Open the path in read-only mode, returns `IoResult<File>`
    let file_desc = match File::open(&path) {
        // The `desc` field of `IoError` is a string that describes the error
        Err(why) => fail!("couldn't open {}: {}", file_display, why.desc),
        Ok(file_desc) => file_desc,
    };

    let mut file_reader = BufferedReader::new(file_desc);

    // skip header lines
    for line in file_reader.lines() {
        let file_data_line = line.unwrap();

        if file_data_line.as_slice() == "%Data\n" {
            break;
        }
    }

    let mut country_reader = csv::Reader::from_reader(file_reader);
    let mut country_record_vector = vec![];

    for country_record_line in country_reader.decode() {
        // Read the line into my country struct
        let country_record: CountryRecord = country_record_line.unwrap();

        country_record_vector.push(country_record);
    }

    country_record_vector
}

fn get_year() -> uint {
    println!("Give me an year from {} to {}.", MIN_YEAR, MAX_YEAR);

    let mut user_option;
    let mut user_year;
    loop {
        user_option = get_user_choice();

        match user_option {
            None => {
                println!("Not a valid year, the year must be in the range of {} and {}", MIN_YEAR, MAX_YEAR);
                continue;
            },
            Some(_) => {
                user_year = user_option.unwrap();

                if user_year < MIN_YEAR || user_year > MAX_YEAR {
                    println!("Not a valid year, the year must be in the range of {} and {}", MIN_YEAR, MAX_YEAR);
                    continue;
                }
                else {break;}
            },
        }
    }
    user_year
}

fn get_geo_area_from_user() -> uint {
    println!("Give me an geo area from {} to {}.", MIN_GEO_AREA_NUMBER, MAX_GEO_AREA_NUMBER);

    let mut user_option;
    let mut user_geo_area_number;
    loop {
        user_option = get_user_choice();

        match user_option {
            None => {
                println!("Not a valid geo area number, the number must be in the range of {} and {}", MIN_GEO_AREA_NUMBER, MAX_GEO_AREA_NUMBER);
                continue;
            },
            Some(_) => {
                user_geo_area_number = user_option.unwrap();

                if user_geo_area_number < MIN_GEO_AREA_NUMBER || user_geo_area_number > MAX_GEO_AREA_NUMBER {
                    println!("Not a valid geo area number, the number must be in the range of {} and {}", MIN_GEO_AREA_NUMBER, MAX_GEO_AREA_NUMBER);
                    continue;
                }
                else {break;}
            },
        }
    }
    user_geo_area_number
}
