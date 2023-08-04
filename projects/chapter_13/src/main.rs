#[derive(Debug, Clone, PartialEq, Copy)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {

    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stock())
    }

    fn most_stock(&self) -> ShirtColor {

        let mut nb_blue = 0;
        let mut nb_red = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => nb_blue += 1,
                ShirtColor::Red => nb_red += 1,
            }
        }

        if nb_red < nb_blue {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {

    let stock = Inventory{shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red, ShirtColor::Red]};
    println!("Most in stock: {:?}", stock.most_stock());

    let user1_pref = Some(ShirtColor::Blue);
    let giveaway1 = stock.give_away(user1_pref);
    println!("User1 with pref {:?} gets {:?}", user1_pref, giveaway1);

    let user2_pref: Option<ShirtColor> = None;
    let giveaway2 = stock.give_away(user2_pref);
    println!("User2 with pref {:?} gets {:?}", user2_pref, giveaway2);

}
