//Dylan Nelson - X00144862

fn main() {
    //3 different Longjumpers using Vector
    let mut array_of_jumpers: Vec<LongJumper> = vec![];

    array_of_jumpers.push(LongJumper{
        name: "Dylan".to_string(),
        gendereventtype : "Male".to_string(),
        personalbestmeters: 12.2,
        bestseasonjump: 9.3,
        noofcompjumps: 20
    });
    array_of_jumpers.push(LongJumper{
        name: "Tara".to_string(),
        gendereventtype : "Female".to_string(),
        personalbestmeters: 9.3,
        bestseasonjump: 8.1,
        noofcompjumps: 36
    });
    array_of_jumpers.push(LongJumper{
        name: "Thomas".to_string(),
        gendereventtype : "Male".to_string(),
        personalbestmeters: 14.1,
        bestseasonjump: 14.1,
        noofcompjumps: 24
    });

     println! {"{:?}",array_of_jumpers}

//Filter using vector  and iterator
     let new_arr = array_of_jumpers
     .iter()
     .filter(|s| s.bestseasonjump == personalbestmeters);
 for elem in new_arr {
     println! {"{:?}", elem};
 }
}

#[derive(Debug)]                   //Struct for LongJumper
struct LongJumper{
    name: String,
    gendereventtype: String,
    personalbestmeters: f64,
    bestseasonjump: f64,
    noofcompjumps: u32
}

// ***Could not get Working was filled with errors ***
impl LongJumper{
 fn newLongJumper(&self,name:String, gendereventtype:String, personalbestmeters: f64, bestseasonjump: f64 ,noofcompjumps:u32) -> LongJumper{
     self.name = name;
     self.gendereventtype = gendereventtype;
    self.personalbestmeters = personalbestmeters;
     self.bestseasonjump = 0;
     self.noofcompjumps = 0;
}
}
