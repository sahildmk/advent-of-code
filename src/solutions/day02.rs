use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

// max of each coloured cube

pub fn day02() -> Result<(), std::io::Error> {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;

    let path = Path::new("src/inputs/day02.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut game_sum = 0;

    for line in reader.lines() {
        let line = line?;
        let mut split_line = line.split(": ");
        let mut cube_max: HashMap<String, u32> = HashMap::new();

        let mut game = split_line.next().unwrap().split(" ");
        let game_id = game.nth(1).unwrap().parse::<u32>().unwrap();
        let cube_groups = split_line.next().unwrap().split("; ");

        cube_groups.for_each(|cube_group| {
            let cubes = cube_group.split(", ");
            cubes.for_each(|cube| {
                let mut cube_split = cube.split(" ");
                let count = cube_split.next().unwrap().parse::<u32>().unwrap();
                let colour = cube_split.next().unwrap();

                if cube_max.contains_key(colour) {
                    let current_max = cube_max.get(colour).unwrap();
                    if count > *current_max {
                        cube_max.insert(colour.to_string(), count);
                    }
                } else {
                    cube_max.insert(colour.to_string(), count);
                }
            });
        });

        if cube_max.get("red").unwrap() <= &red_max
            && cube_max.get("green").unwrap() <= &green_max
            && cube_max.get("blue").unwrap() <= &blue_max
        {
            game_sum += game_id;
        }
    }

    println!("Game sum: {}", game_sum);

    Ok(())
}
