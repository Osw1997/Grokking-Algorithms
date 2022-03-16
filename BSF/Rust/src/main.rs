// Implementation of Breadth Search First (BSF)
use std::collections::VecDeque;

fn main() {

    let mut me = vec!["mike", "luis", "lalo"];
    let mut mike: Vec<&str> = vec![];
    let mut luis: Vec<&str> = vec![];    
    let mut lalo = vec!["pancho", "esau"];

    let mut pancho = vec!["merlin", "mike", "uzi"];
    let mut esau: Vec<&str> = vec![];
    let mut merlin: Vec<&str> = vec![];
    let mut uzi: Vec<&str> = vec![];


    // Char to look for
    let char_to_find = 'u';


    /*
         Lets find the person whose name finishes in "i" (char_to_find)
    */
    // Visited users
    let mut visited_friends: Vec<&str> = Vec::new();

    // Lets look in my friends
    let mut queue = VecDeque::new();
    let mut current_friend = "me";
    loop {
        // Super dirty way to implement my dictionary
        // Intermediate variable that allows us to load the friend's friedns to the QUEUE
        let mut friend_list: Vec<&str> = vec![];
        if current_friend == "me" {
            friend_list = me.clone();
        }
        if current_friend == "mike" {
            friend_list = mike.clone();
        }
        if current_friend == "luis" {
            friend_list = luis.clone();
        }
        if current_friend == "lalo" {
            friend_list = lalo.clone();
        }

        if current_friend == "pancho" {
            friend_list = pancho.clone();
        }
        if current_friend == "esau" {
            friend_list = esau.clone();
        }
        if current_friend == "merlin" {
            friend_list = merlin.clone();
        }
        if current_friend == "uzi" {
            friend_list = uzi.clone();
        }
        
        // Let's check if current friend has not been visited previously.
        let mut friend_pop = "";
        if !visited_friends.contains(&current_friend) {
            // Let's add friend list to the QUEUE
            for friend in friend_list {
                queue.push_back(friend);
            }
            
            // Lets check if current friend's name finishes in "i"
            // let friend_pop = queue.pop_front().unwrap();
            friend_pop = queue.pop_front().unwrap();
            let last_char = friend_pop.chars().last().unwrap();


            println!("Does {} => [{}] finish in '{}'? {}", friend_pop, last_char, char_to_find, last_char == char_to_find);
            if last_char == char_to_find {
                break;
            }
        }

        // Let's add current user to the visited_friends list;
        visited_friends.push(current_friend);
        // Lets UPDATE current friend
        current_friend = friend_pop;

        println!("Visited friends: {:?}", visited_friends);
        
    }
    
}
