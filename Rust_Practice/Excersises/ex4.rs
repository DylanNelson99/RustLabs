// Using this derived annotation in order to
// print out the variants (values) of the enum
#[derive(Debug)]
enum Days_of_the_week {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// Methods that you can create for the enum struct
impl Days_of_the_week {
    fn weekday_or_weekend(&self) -> String {
        // match is similar to a switch statement
        match &self {
            // First condition or arm
            // checks if the ref variable has any variant
            // value within this, return "Week day"
            Days_of_the_week::Monday
            | Days_of_the_week::Tuesday
            | Days_of_the_week::Wednesday
            | Days_of_the_week::Thursday => "Week day".to_string(),
            // Here we check if the ref variable has any variant value
            // within this, return "week end"
            Days_of_the_week::Friday | Days_of_the_week::Saturday | Days_of_the_week::Sunday => {
                "Week end".to_string()
            },
            _ => "No day"
        }
    }
}

let m: Days_of_the_week = Days_of_the_week::Monday;
println! {"{}", m.weekday_or_weekend()};