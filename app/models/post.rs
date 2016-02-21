use std::fmt;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Post {
    id: i64,
    pub author: String,
    pub date: String,
    pub title: String,
    pub content: String
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Post {
    pub fn get_by_id(id: i64) -> Post {
        Post {
            id: id,
            author: "Bryan Mytko".to_string(),
            date: "February 19, 2016".to_string(),
            title: "Cat Ipsum".to_string(),
            content: "Instantly break out into full speed gallop across the house for no reason run
            in circles ignore the squirrels, you'll never catch them anyway chew on cable so ears
            back wide eyed attack dog, run away and pretend to be victim. Chew on cable plan steps
            for world domination but hide head under blanket so no one can see yet favor packaging
                over toy, so give attitude, play riveting piece on synthesizer keyboard. Sleep nap
                    chase the pig around the house. Hack up furballs favor packaging over toy has
                    closed eyes but still sees you. Lick yarn hanging out of own butt. Vommit food
                    and eat it again hunt anything that moves leave hair everywhere, but destroy
                    couch as revenge yet stare out the window paw at your fat belly. Stare at
                    ceiling. Poop in the plant pot chase imaginary bugs.".to_string()
        }
    }
}
