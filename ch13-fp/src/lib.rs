use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum ShirtColor {
    Blue,
    Red,
}

struct Inventory {
    shirts: HashMap<ShirtColor, i32>,
}

impl Inventory {

    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let num_blue = self.shirts.get(&ShirtColor::Blue).unwrap_or(&0);
        let num_red = self.shirts.get(&ShirtColor::Red).unwrap_or(&0);
        if num_blue >= num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn giveaway_most_stocked() {
        let store = Inventory {
            shirts: HashMap::from([
                (ShirtColor::Blue, 20),
                (ShirtColor::Red, 30),
            ])
        };
        assert_eq!(store.giveaway(None), ShirtColor::Red); 
    }

    #[test]
    fn giveaway_preferred() {
        let store = Inventory {
            shirts: HashMap::from([
                (ShirtColor::Blue, 20),
                (ShirtColor::Red, 30),
            ])
        };
        assert_eq!(store.giveaway(Some(ShirtColor::Blue)), ShirtColor::Blue); 
    }

}
