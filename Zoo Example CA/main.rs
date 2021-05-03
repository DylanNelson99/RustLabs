#[derive(Debug, PartialEq)]
enum Ticket_category {
    Infant,
    Child,
    Adult,
    OAP,
}

fn main() {
    println! {"{:?}", age_category(10)};
    println! {"{:?}", price_ticket(age_category(10))};

    let mut array_of_person: Vec<Person> = vec![];

    array_of_person.push(Person {
        name: "Nitesh - Child 1".to_string(),
        age: 12,
    });
    array_of_person.push(Person {
        name: "Wyd - Step Sis".to_string(),
        age: 10,
    });
    array_of_person.push(Person {
        name: "Michael - Dad".to_string(),
        age: 56,
    });
    array_of_person.push(Person {
        name: "Lana Rhoades - Mom".to_string(),
        age: 33,
    });
    println! {"{:?}", array_of_person};
    let total_final_price = array_of_person.iter().fold(0.0, |total_price, i| {
        total_price + price_ticket(age_category(i.age))
    });
    println! {"{}", total_final_price};

    let new_family = array_of_person
        .iter()
        .filter(|c| age_category(c.age) == Ticket_category::Child)
        .collect::<Vec<&Person>>();

    for elem in new_family {
        println! {"Name -> {}", elem.name};
        println! {"Age -> {}", elem.age};
    }
}

fn age_category(age: u32) -> Ticket_category {
    match age {
        0..=2 => Ticket_category::Infant,
        3..=16 => Ticket_category::Child,
        17..=65 => Ticket_category::Adult,
        _ => Ticket_category::OAP,
    }
}

fn price_ticket(category: Ticket_category) -> f32 {
    match category {
        Ticket_category::Infant => 0.,
        Ticket_category::Child => 13.00,
        Ticket_category::Adult => 17.50,
        Ticket_category::OAP => 13.50,
    }
}

// Structure for a person
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
