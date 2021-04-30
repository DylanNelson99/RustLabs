#[derive(Debug, PartialEq)]
enum Subscription {
    Basic,
    Standard,
    Premium,
}
#[derive(Debug)]
enum PictureDefinition {
    Standard,
    HighDefinition,
    UltraHighDefinition,
}
fn main() {
    let mut subscriptions: Vec<Subscriptions> = vec![];
    subscriptions.push(Subscriptions {
        subcription_type: Subscription::Basic,
        max_screens: 1,
        picture_definition: PictureDefinition::Standard,
        price: 7.99,
    });
    subscriptions.push(Subscriptions {
        subcription_type: Subscription::Premium,
        max_screens: 3,
        picture_definition: PictureDefinition::HighDefinition,
        price: 10.99,
    });
    subscriptions.push(Subscriptions {
        subcription_type: Subscription::Standard,
        max_screens: 1,
        picture_definition: PictureDefinition::UltraHighDefinition,
        price: 15.99,
    });
    subscriptions.push(Subscriptions {
        subcription_type: Subscription::Premium,
        max_screens: 4,
        picture_definition: PictureDefinition::UltraHighDefinition,
        price: 7.99,
    });

    let basic_sub = subscriptions
        .iter()
        .find(|s| s.subcription_type == Subscription::Premium);

    println! {"{:?}", basic_sub};

    let sub_less_than_ten = subscriptions.iter().any(|s| s.price < 10.00);
    println! {"{}", sub_less_than_ten};

    let new_vec = subscriptions.iter().map(|s| s.price * 1.21);
    for elem in new_vec {
        println! {"{:?}", elem};
    }
}

#[derive(Debug)]
struct Subscriptions {
    subcription_type: Subscription,
    max_screens: u32,
    picture_definition: PictureDefinition,
    price: f32,
}
