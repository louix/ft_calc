use std::error::Error;
use std::fs;

mod crops {
    use serde::Deserialize;
    use serde_json::error::Error as SerdeError;

    #[derive(Deserialize, Debug)]
    pub struct Crop {
        name: String,
        cost: u32,
        time: u32,
        sale_price: u32,
    }

    impl Crop {
        fn new(name: String, cost: u32, time: u32, sale_price: u32)
        -> Self {
            Self {
                name,
                cost,
                time,
                sale_price,
            }
        }
        pub fn name<'a>(&'a self) -> &'a str {
            &self.name
        }
        pub fn cost<'a>(&'a self) -> &'a u32 {
            &self.cost
        }
        pub fn time<'a>(&'a self) -> &'a u32 {
            &self.time
        }
        pub fn sale_price<'a>(&'a self) -> &'a u32 {
            &self.sale_price
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

    pub fn load_from_json(json_data: &str) -> Result<Vec<Crop>, SerdeError> {
        serde_json::from_str(json_data)
    }

    pub fn get_by_efficiency(crops: &Vec<Crop>, time: u32, money: u32) -> Vec<(&Crop, u32, u32)> {
        let mut viable_crops: Vec<(&Crop, u32, u32)> = crops.into_iter()
                                .filter(|x| x.time <= time && x.cost <= money)
                                // Add how many we can buy:
                                .map(|x| {
                                    let mut purchaseable_amount = money / (x.cost + PLOW_COST);
                                    if purchaseable_amount > 200 { purchaseable_amount = 200 };
                                    let profit = (x.sale_price - x.cost - PLOW_COST) * purchaseable_amount;
                                    (x, purchaseable_amount, profit)
                                })
                                .collect();

        // Order by profit
        viable_crops.sort_by(|a, b| a.2.cmp(&b.2).reverse());
        viable_crops
    }

    pub fn get_highest_sale_price(crops: &Vec<Crop>) -> &Crop {
        let mut highest = &crops[0];

        for crop in crops.iter() {
            if crop.sale_price > highest.sale_price {
                highest = crop;
            }
        }
       &highest
    }

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
            let loaded_crops = load_from_json(JSON_TEST_DATA)?;

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
            let actual_highest = get_highest_sale_price(&test_crops);
            assert_eq!(actual_highest, expected_highest);
        }

        #[test]
        fn most_efficient_crop_is_selected() {
            let test_crops = get_test_crops();
            let most_efficient_crops = get_by_efficiency(&test_crops, 20, 200);
            assert_eq!(most_efficient_crops.len(), 1);
            assert_eq!(most_efficient_crops[0].0, &test_crops[0]);
        }
    }
}

pub fn run(money: u32, time: u32) -> Result<(), Box<dyn Error>> {
    let crops_contents = fs::read_to_string("crops.json")?;
    let crops = crops::load_from_json(&crops_contents)?;

    let crops = crops::get_by_efficiency(&crops, time, money);

    for crop in crops {
        println!("Crop: {}\nCount: {}\nProfit: {}\n",
            crop.0.name(), crop.1, crop.2);
    }

    Ok(())
}
