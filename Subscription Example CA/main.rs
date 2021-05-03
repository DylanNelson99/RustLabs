fn main() {
    let mut subscription: Vec<Subscription> = vec![];

    // basic
    subscription.push(Subscription {
        subscription_type: SubType::Basic,
        screens: 1,
        picture_definition: PicDef::SD,
        price: 7.99,
    });

    // Standard
    subscription.push(Subscription {
        subscription_type: SubType::Standard,
        screens: 2,
        picture_definition: PicDef::HD,
        price: 10.99,
    });

    //Premium
    subscription.push(Subscription {
        subscription_type: SubType::Premium,
        screens: 4,
        picture_definition: PicDef::UltraHD,
        price: 15.99,
    });

    println! {"->Print of all Sub Types: {:?} \n",subscription}

    match subscription
        .iter()
        .find(|&x| x.subscription_type == SubType::Premium)
    {
        None => println!("none"),
        Some(premium) => println!("->Finding Premium Sub: {:?} \n", premium),
    }

    //Any subscription less than 10
    let cost = 10.00;
    assert_eq!(subscription.iter().any(|x| x.price < cost), true);

    //Changing for euro to dollars on prices
    let dollar = subscription.iter().map(| s | s.price * 1.1).collect::<Vec<f64>>();
    println! {"->Subscription prices in dollars:(Basic/Standard/Premium) {:?}", dollar};
}

//Subscription Type
#[derive(Debug, PartialEq)]
enum SubType {
    Basic,
    Standard,
    Premium,
}

//Movie Quality
#[derive(Debug, PartialEq)]
enum PicDef {
    SD,
    HD,
    UltraHD,
}

//Subscription Structure
#[derive(Debug, PartialEq)]
struct Subscription {
    subscription_type: SubType,
    screens: u32,                       //max number of sceens for viewing
    picture_definition: PicDef,
    price: f64,                         //price of subscription per month
}
