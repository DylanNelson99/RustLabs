//Dylan Nelson - X00144862

fn main() {
    //Update each jumper
    let mut mp = LongJumper::new(String::from("Dylan Nelson"), EventType::Mens, 8.5);
    mp.update_stats(8.2);
    mp.update_stats(8.1);
    mp.update_stats(8.6);

    let mut tb = LongJumper::new(String::from("Tara MacCourt"), EventType::Womens, 7.17);
    tb.update_stats(7.0);
    tb.update_stats(6.7);

    let mut br = LongJumper::new(String::from("Roisin Brien"), EventType::Womens, 7.0);
    br.update_stats(7.12);
    br.update_stats(6.9);

    //Add jumpers to a vector
    let mut jumpers: Vec<LongJumper> = vec![];
    jumpers.push(mp);
    jumpers.push(tb);
    jumpers.push(br);

    //Display jumpers stats
    println!("Jumpers {:?}", jumpers);

    // get jumpers who have had a PB in this season
    let seasons_best = jumpers
        .iter()
        .filter(|lj| lj.seasons_best == lj.personal_best) //using iterator and filter method
        .collect::<Vec<&LongJumper>>();
    println!("PB this season {:?}", seasons_best);

    for i in 0..jumpers.len() -1 { //Display jumpers whos PB was in this season
        jumpers[i].close_season();
    }
}

#[derive(Clone, Debug)]
enum EventType {
    Mens,
    Womens,
}

#[derive(Debug)]                    //Struct for a Long Jumper
struct LongJumper {
    name: String,
    gender: EventType,              // Male or Female
    personal_best: f64,             // in metres
    seasons_best: f64,              // in metres
    season_jumps: u32,              // no of jumps in this season
}

impl LongJumper {                   //implementation of Long jumper function to allow for update
    fn new(name: String, gender: EventType, personal_best: f64) -> LongJumper {
        LongJumper {
            name,
            gender,
            personal_best,
            seasons_best: 0.,        // set to 0.0
            season_jumps: 0,        // set to 0
        }
    }

  
    fn update_stats(&mut self, jump: f64) {   // update stats for season
        if jump > self.personal_best {
            self.personal_best = jump;
            self.seasons_best = jump;
        } else if jump > self.seasons_best {
            self.seasons_best = jump;
        }
        self.season_jumps += 1;
    }

    fn close_season(&mut self) {     // close the season
        self.seasons_best = 0.;
        self.season_jumps = 0;
    }
}
