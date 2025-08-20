/*
Easy way to generate the buy packages
I'm sure this can be improved in some way but honestly it's one file
*/

use std::fs::File;
use std::io::Write;

const MAX_WEALTH: usize = 99;
// From 1 to this number
const DIVISION: u32 = 1;         
// Increase division so that we can have a larger world population but not larger demand

fn main() {

    let mut file = File::create("output.txt")
        .expect("How did it fail to create a file????");

    let mut wealth_level: u32 = 1;
    let pol_strength = |x: u32| -> f32 { if x < 30 {
        ((x as f32) * 0.3).powi(2)
        } else { 100. + (25 * (x-30)) as f32 } };
    let goods: Vec<(&str, fn(u32) -> u32)> = vec![
        ("popneed_live_at_home",    |_x| {  5                                       } ),
        ("popneed_leisure",         | x| {  5 + ((5_u32).pow(2) * x ).isqrt()       } ),
        ("popneed_social",          | x| { 20 + ( x * 2 )                           } ),
        ("popneed_media",           | x| { match x {
                                          0..5 => 0,              // No media needed below 5
                                          _    => (x - 5) * 5                     } } ),
        ("popneed_comfort",         | x| { 5 + x.saturating_sub(5) * 3              } ),
        ("popneed_household_items", | x| { 10 + (x * 10)                            } ),
        ("popneed_movement",        | x| { 50 + (x * 5).saturating_sub(100)         } ),
        ("popneed_climate_control", | x| { 50 + (x * 3).saturating_sub(50)          } ),
        ("popneed_nutrition",       |_x| { 50                                       } ),
        ("popneed_service_finance", | x| { (5 * x).saturating_sub(50)               } ), 
        ("popneed_healthcare",      | x| { match x {
                                         0..5 => 20,
                                         _    => 20 + (( x - 5 ).pow(2))          } } ),
        ("popneed_vices",           | x| { match x {
                                         0..3  => 0,
                                         3..11 => (x - 3).pow(3),  
                                         _     => (343_u32)         
                                            .saturating_sub(x * 5)  
                                            .clamp(100, u32::MAX)                 } } ),
        ("popneed_clothing",        |_x| { 50                                       } ),
        ("popneed_utilities",       | x| { 30 * x.clamp(0, 2000)                    } ),

        ];

    let mut file_contents = String::new();
    for i in 1..=MAX_WEALTH {
        let return_goods = return_goods(wealth_level, &goods);
        file_contents.push_str( format!(
r"wealth_{} = {{
    political_strength = {:.2}
        goods = {{ {}
    }}
}}
"       ,wealth_level, pol_strength(wealth_level), return_goods
        ).as_str());
        wealth_level += 1;
    }
    file.write_all(file_contents.as_bytes()).expect("Error writing contents");
}

fn return_goods(wealth_level: u32, goods: &Vec<(&str, fn(u32) -> u32)>) -> String {
    let mut result: String = String::new();
    for i in goods {
        if i.1(wealth_level) == 0 { continue; }     // Guard so popneed = 0 just doesn't pop up
        result.push_str(
            format!(
                "\n{:8}{} = {}","",
                i.0, i.1(wealth_level)
                    .checked_div(DIVISION)
                    .unwrap_or(0))
                    .as_str()
            )
    }
    result
}