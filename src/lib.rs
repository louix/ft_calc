use std::fs;
use std::error::Error;
use structopt::StructOpt;
use serde::Deserialize;
use serde_json::error::Error as SerdeError;

#[derive(StructOpt)]
#[structopt(
    name = "Farm Together Calculator",
    about = "A tool to caluclate the best crop to plant, ready to harvest in the given amount of time."
)]
struct Config {
    #[structopt(help = "Amount of money you can use for planting the crops")]
    money: u32,
    #[structopt(help = "Duration in minutes. After this duration your crops will be ready to harvest")]
    time: u32,
}

pub struct Farm {
    crops: Vec<Crop>
}

impl Farm {
    pub fn from_json() -> Result<Farm, Box<dyn Error>> {
        let crops_contents = fs::read_to_string("crops.json")?;
        Ok(
            Farm {
                crops: Crop::from_json(&crops_contents)?
            }
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct Crop {
    name: String,
    cost: u32,
    time: u32,
    sale_price: u32,
}

impl Crop {
    pub fn from_json(json_data: &str) -> Result<Vec<Crop>, SerdeError> {
        serde_json::from_str(json_data)
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn cost(&self) -> &u32 {
        &self.cost
    }
    pub fn time(&self) -> &u32 {
        &self.time
    }
    pub fn sale_price(&self) -> &u32 {
        &self.sale_price
    }
    pub fn filter_by_efficiency(crops: &[Crop], time: u32, money: u32) -> Vec<(&Crop, u32, u32)> {
        let mut viable_crops: Vec<(&Crop, u32, u32)> = crops.iter()
                                .filter(|x| x.time <= time && x.cost <= money)
                                // Add how many we can buy:
                                .map(|x| {
                                    let mut purchaseable_amount = money / (x.cost + PLOW_COST);
                                    if purchaseable_amount > MAX_CROP_COUNT { purchaseable_amount = MAX_CROP_COUNT };
                                    let profit = (x.sale_price - x.cost - PLOW_COST) * purchaseable_amount;
                                    (x, purchaseable_amount, profit)
                                })
                                .collect();

        // Order by profit
        viable_crops.sort_by(|a, b| a.2.cmp(&b.2).reverse());
        viable_crops
    }

    pub fn get_highest_sale_price(crops: &[Crop]) -> &Crop {
        let mut highest = &crops[0];

        for crop in crops.iter() {
            if crop.sale_price > highest.sale_price {
                highest = crop;
            }
        }
        &highest
    }
}

impl PartialEq for Crop {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.cost == other.cost &&
        self.time == other.time &&
        self.sale_price == other.sale_price
    }
}

const PLOW_COST: u32 = 10;
const MAX_CROP_COUNT: u32 = 200;

#[cfg(test)]
mod test {
    // Load (parent) library code
    use super::*;
    use std::error::Error;

    const JSON_TEST_DATA: &str = r#"[
        {
            "name": "Lettuce",
            "cost": 15,
            "time": 10,
            "sale_price": 30
        },
        {
            "name": "Leek",
            "cost": 1250,
            "time": 45,
            "sale_price": 1380
        }
    ]"#;

    fn get_test_crops() -> Vec<Crop> {
        vec![
            Crop {
                name: String::from("Lettuce"),
                cost: 15,
                time: 10,
                sale_price: 30,
            },
            Crop {
                name: String::from("Leek"),
                cost: 1250,
                time: 45,
                sale_price: 1380,
            }
        ]
    }

    #[test]
    fn config_loaded_from_json() -> Result<(), Box<dyn Error>> {
        let test_crops = get_test_crops();
        let loaded_crops = Crop::from_json(JSON_TEST_DATA)?;

        let crops = test_crops.iter().zip(loaded_crops.iter());

        for crop in crops {
            assert_eq!(crop.0, crop.1);
        }

        Ok(())

    }

    #[test]
    fn highest_sale_price_is_correct() {
        let test_crops = get_test_crops();
        let expected_highest = &test_crops[1];
        let actual_highest = Crop::get_highest_sale_price(&test_crops);
        assert_eq!(actual_highest, expected_highest);
    }

    #[test]
    fn most_efficient_crop_is_selected() {
        let test_crops = get_test_crops();
        let most_efficient_crops = Crop::filter_by_efficiency(&test_crops, 20, 200);
        assert_eq!(most_efficient_crops.len(), 1);
        assert_eq!(most_efficient_crops[0].0, &test_crops[0]);
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let farm = Farm::from_json()?;
    let config = Config::from_args();

    let crops = Crop::filter_by_efficiency(&farm.crops, config.time, config.money);

    for crop in crops {
        println!("Crop: {}\nCount: {}\nProfit: {}\n",
            crop.0.name(), crop.1, crop.2);
    }

    Ok(())
}
