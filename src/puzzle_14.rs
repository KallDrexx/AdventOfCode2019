use std::collections::HashMap;

#[derive(Debug)]
struct Recipe {
    inputs: HashMap<String, u128>,
    output: String,
    quantity: u128,
}

pub fn run() {
    let recipes = get_input();
    let mut max_fuel_count = 0;
    let max_ore_count = 1000000000000;

    let mut min = 0;
    let mut max = max_ore_count;
    loop {
        let fuel_size = min + ((max - min) / 2);
        let mut inventory = HashMap::new();
        let mut ore_count = 0;

        create_element_batch(&"FUEL".to_owned(), fuel_size, &recipes, &mut inventory, &mut ore_count);

        println!("{} ({} {}) = {}", fuel_size, min, max, ore_count);

        if ore_count >= max_ore_count {
            max = fuel_size;
            if max - min == 1 {
                max = min;
            }
        } else {
            if max_fuel_count < fuel_size {
                max_fuel_count = fuel_size;
            }

            min = fuel_size;
            if max - min == 1 {
                min = max;
            }
        }

        if min == max {
            break;
        }
    }

    println!("Max fuel: {}", max_fuel_count);
}

fn get_input() -> HashMap<String, Recipe> {
    let input = "1 XVCBM, 12 SWPQ => 7 VMWSR
10 SBLTQ, 14 TLDR => 6 HJFPQ
1 VWHXC, 2 GZDQ, 3 PCLMJ => 4 VJPLN
9 MGVG => 7 WDPF
1 FBXD, 5 FZNZR => 6 GZDQ
5 TJPZ, 1 QNMZ => 5 SWPQ
12 XWQW, 1 HJFPQ => 8 JPKNC
15 CPNC, 2 TXKRN, 2 MTVQD => 9 LBRSX
5 VJPLN, 1 VSTRK, 2 GFQLV => 5 NLZKH
1 TLDR => 4 TNRZW
2 VCFM => 7 FZNZR
1 PSTRV, 5 RTDV => 8 VCFM
2 PSTRV => 9 SFWJG
4 XWQW => 2 BHPS
1 ZWFNW, 19 JKRWT, 2 JKDL, 8 PCLMJ, 7 FHNL, 22 MSZCF, 1 VSTRK, 7 DMJPR => 1 ZDGF
22 XVCBM, 8 TBLM => 1 MTVQD
101 ORE => 1 WBNWZ
6 VNVXJ, 1 FBXD, 13 PCLMJ => 9 MGVG
13 SHWB, 1 WDPF, 4 QDTW => 6 FHNL
9 VSTRK => 2 VZCML
20 LZCDB => 7 KNPM
2 LBRSX, 9 GRCD => 3 SHWB
5 BHPS => 6 SQJLW
1 RTDV => 6 GRCD
6 SBLTQ, 6 XWQW => 5 CPNC
153 ORE => 3 RTDV
6 LZCDB, 1 SBLTQ => 3 PCLMJ
1 RTDV, 2 TJPZ => 5 LZCDB
24 QNMZ => 4 TXKRN
19 PCLMJ, 7 VNVXJ => 6 RKRVJ
12 RKRVJ, 11 QNMZ => 3 JKRWT
4 SFWJG => 9 FBXD
16 WDPF, 4 TXKRN => 6 DMJPR
3 QNMZ => 1 VSTRK
9 VSTRK => 4 ZWFNW
7 QBWN, 1 TLDR => 4 QDTW
7 VJPLN, 1 NLZKH, 15 JPKNC, 3 SHWB, 1 MSZCF, 3 VMWSR => 6 QDHGS
14 QXQZ => 7 XWQW
152 ORE => 9 TJPZ
1 PJVJ, 10 QBWN, 19 NLZKH => 6 MSZCF
21 TLDR, 13 VNVXJ, 5 BHPS => 4 QBWN
1 GZDQ, 6 GRCD => 9 TLDR
4 BHPS => 8 MZBL
1 FZNZR => 2 VNVXJ
1 VNVXJ => 5 GFQLV
13 LZCDB => 2 QXQZ
3 MNFJX => 5 VWHXC
1 GZDQ, 2 VMWSR => 6 WZMHW
9 HJFPQ, 3 RKRVJ => 4 QNMZ
8 TJPZ => 9 SBLTQ
30 WBNWZ => 5 TBLM
1 PCLMJ => 3 GNMTQ
30 SQJLW, 3 QNMZ, 9 WDPF => 5 PJVJ
10 GRCD, 15 SBLTQ, 22 GFQLV => 4 XVCBM
30 PJVJ, 10 JPKNC, 3 DXFDR, 10 VZCML, 59 MZBL, 40 VWHXC, 1 ZDGF, 13 QDHGS => 1 FUEL
4 GNMTQ, 6 VMWSR, 19 RKRVJ, 5 FKZF, 4 VCFM, 2 WZMHW, 7 KNPM, 5 TNRZW => 7 DXFDR
152 ORE => 9 PSTRV
2 BHPS, 5 TXKRN, 2 PJVJ => 4 FKZF
2 XWQW, 2 VCFM, 13 BHPS => 8 MNFJX
3 XWQW => 2 JKDL
";

    let mut result = HashMap::new();
    result.insert("ORE".to_string(), Recipe {inputs: HashMap::new(), output: "ORE".to_string(), quantity: 1});
    for line in input.lines() {
        let mut inputs = HashMap::new();
        let sections: Vec<&str> = line.split(|x| x == ',' || x == '=').collect();
        for section in sections {
            if section.starts_with(">") {
                // output
                let parts: Vec<&str> = section.split(' ').collect();
                let count = parts[1].parse().unwrap();
                let name = parts[2];
                result.insert(name.to_owned(), Recipe {
                    inputs: inputs.clone(),
                    output: name.to_string(),
                    quantity: count,
                });
            } else {
                // input
                let parts: Vec<&str> = section.split(' ').collect();
                let start_index = if parts[0] == "" { 1 } else { 0 };
                let number = parts[start_index].parse().unwrap();
                let name = parts[start_index + 1];
                inputs.insert(name.to_owned(), number);
            }
        }
    }

    result
}

fn create_element_batch(element: &String,
                        batch_size: u128,
                        recipes: &HashMap<String, Recipe>,
                        inventory: &mut HashMap<String, u128>,
                        ore_count: &mut u128) {
    let recipe = match recipes.get(element) {
        None => panic!("No known recipe for '{}'", element),
        Some(x) => x,
    };

    for (input, input_quantity_needed) in &recipe.inputs {
        let available_count = match inventory.get(input) {
            None => 0,
            Some(x) => *x,
        };

        let total_needed = batch_size * *input_quantity_needed;
        let batch_output_size = match recipes.get(input) {
            None => panic!("{} does not have a recipe", input),
            Some(x) => x.quantity,
        };

        if total_needed > available_count {
            let additional = total_needed - available_count;
            let mut batch_count = additional / batch_output_size;
            if additional % batch_output_size > 0 {
                batch_count = batch_count + 1;
            }

            //println!("{} more {} needed ({} batches)", additional, input, batch_count);
            create_element_batch(input, batch_count, recipes, inventory, ore_count);
        }

        inventory.insert(input.clone(), inventory[input] - total_needed);
    }

    let current = match inventory.get(element) {
        None => 0,
        Some(x) => *x,
    };

    let created_count = recipe.quantity * batch_size;
    inventory.insert(element.clone(), current + created_count);
    //println!("{} {} created ({})", created_count, element, inventory[element]);

    if *element == "ORE".to_string() {
        *ore_count = *ore_count + created_count;
    }
}

