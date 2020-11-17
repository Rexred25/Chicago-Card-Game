use deckofcards::{Cards, Deck, Hand};
use std::io;

fn deal_cards(pass_real_score: &mut [i64; 3], pass_player_turn: &mut i64, pass_game_round: &mut i64) {
    let mut deck = Deck::new();
    let mut hand1 = Hand::new();
    let mut hand2 = Hand::new();
    let mut hand3 = Hand::new();
    let mut discard = Hand::new();
    let mut hand1_down: Hand = Hand::new();
    let mut hand2_down: Hand = Hand::new();
    let mut hand3_down: Hand = Hand::new();

    let player: [Hand; 3] = [hand1, hand2, hand3]; //lets it be modular
    let player_down: [Hand; 3] = [hand1_down, hand2_down, hand3_down]; //lets it be modular

    let mut may_i: [i64; 3] = [3, 3, 3];

    //deck.shuffle();
    let mut real_score = pass_real_score;
    let mut player_turn = pass_player_turn;
    let mut game_round = pass_game_round;
    player_turn_options(
        &mut deck,
        &mut discard,
        &mut real_score,
        &mut player_turn,
        player,
        player_down,
        &mut game_round,
        may_i,
    );
}

fn player_turn_options(
    //basically this manages all the player options, probably should have functions inside it
    deck: &mut Deck,
    mut discard: &mut Hand,
    //mut score: [i64; 3],
    mut real_score: &mut [i64; 3],
    mut player_turn: &mut i64,
    mut player: [Hand; 3],
    mut player_down: [Hand; 3],
    mut game_round: &mut i64,
    mut may_i: [i64; 3],
) {
    //let mut player_turn: i64 = 1;
    let mut player_out: bool = false;
    let mut player_put_down: bool = true;
    let mut player_first_move = false;

    //println!("cards undealt: {}", deck.undealt_count());
    let mut x = 1;
    for y in 0..3 {
        deck.deal_to_hand(&mut player[y], 11);
        println!("Player {} Hand: {}", x, player[y]);
        x += 1;
    }
    //println!("cards undealt after dealt: {}", deck.undealt_count());

    while player_out != true {
        let mut input = String::new();

        let mut k = *player_turn as usize;
        k -= 1;

        let mut player_discard = false;
        println!("-----------------------------");
        println!("Player {} Hand: {}", player_turn, player[k]);
        println!("discard pile: {}", discard);
        if player_put_down == true {
            println!("-----------------------------");
            println!("\tPlayer Cards Down:");
            let mut b = 1;
            for j in 1..4 {
                print!("Player {} Down: ", j);
                if b == 4 {
                    println!("");
                    break;
                }
                b = j;
                b -= 1;
                println!("hand: {}", player_down[b]);
                b += 1;
            }
            //println!("-----------------------------");
        }

        if player_first_move == false {
            println!("-----------------------------");
            println!("1:\t Draw a card");
            println!("3:\t Pick up card from discard pile");
            println!("5:\t Sort your hand ascending");
            println!("6:\t Sort your hand descending");
            println!("7:\t Check current scores");
            println!("8:\t Someone wants to May I:");
            println!("9:\t Rules");
            println!("0:\t End the program test");
            println!("-----------------------------");
            
        } else {
            println!("-----------------------------");
            println!("2:\t Discard a card and end your turn");
            println!("4:\t Place your cards down");
            println!("5:\t Sort your hand ascending");
            println!("6:\t Sort your hand descending");
            println!("7:\t Check current scores");
            println!("9:\t Rules");
            println!("0:\t End the program test");
            println!("-----------------------------");
        }
        io::stdin().read_line(&mut input).unwrap();
        let mut n: i32 = input.trim().parse().unwrap();
        if n == 8
        {
            player_first_move = false;
        }
        else
        {
            player_first_move = true;
        }
        println!("-----------------------------");

        let mut may_i_trigger = 0;
        if n == 8
        {
            println!("What player would like to May I:");
            let mut b = 1;
            for j in 1..4 {
                print!("Player {} ", j);
                if b == 4 {
                    println!("");
                    break;
                }
                b = j;
                b -= 1;
                println!("May I's Left: {}", may_i[b]);
                b += 1;
            }
            may_i_trigger = 1;
            n = 3;
            
        }
        if n == 1 {
            deck.deal_to_hand(&mut player[k], 1);
        }
        if n == 2 {
            println!("Player {} Hand: {}", player_turn, player[k]);
            println!("What card would you like to discard (Ex. AS)?");
            player_discard = true;
            player_first_move = false;
            n = 4;
            
        }
        if n == 3 {
            if discard.cards.is_empty() {
                println!("Sorry the discard pile is empty.");
            } else {
                //println!("discard pile: {}", discard);
                if may_i_trigger == 1
                {
                    let mut player_request = String::new();
                    io::stdin().read_line(&mut player_request).unwrap();
                    let mut r: i32 = player_request.trim().parse().unwrap();
                    let mut p = r as usize;
                    p -= 1;
                    player[p].push_card(discard.remove(0)); //removes a card from the discard pile and then puts it in the hand
                    deck.deal_to_hand(&mut player[p], 1);
                    may_i[p] -= 1;
                }
                else
                {
                    player[k].push_card(discard.remove(0)); //removes a card from the discard pile and then puts it in the hand
                }
                
            }
            
        }
        if n == 4 {
            if player_discard == true
            {
                let mut put_down = card_put_down();
                
                remove_card(
                    &mut put_down,
                    &mut player,
                    k,
                    player_discard,
                    &mut discard,
                    *player_turn,
                    &mut real_score,
                    &mut player_down,
                    player_put_down,
                    n,
                );
                n = 7;
                *player_turn += 1;
                
            }
            else{
                let mut set_counter = 0;
                let mut run_counter = 0;
                if *game_round == 1 {
                    set_counter = 2;
                    run_counter = -1;

                    let mut x = 1;
                    while x <= set_counter {
                        println!("Enter set {}:", x);
                        let mut put_down = card_put_down();
                        if valid_put_down(&mut put_down, set_counter, run_counter) == false {
                            //cards are not a run or a set
                            println!("Sorry, the cards you entered are not valid.")
                        } else {
                            remove_card(
                                &mut put_down,
                                &mut player,
                                k,
                                player_discard,
                                &mut discard,
                                *player_turn,
                                &mut real_score,
                                &mut player_down,
                                player_put_down,
                                n,
                            );
                        }
                        x += 1;
                    }
                }
                if *game_round == 2 {
                    run_counter = 2;
                    set_counter = -1;

                    let mut x = 1;
                    while x <= run_counter {
                        println!("Enter run {}:", x);
                        let mut put_down = card_put_down();
                        if valid_put_down(&mut put_down, set_counter, run_counter) == false {
                            //cards are not a run or a set
                            println!("Sorry, the cards you entered are not valid.")
                        } else {
                            remove_card(
                                &mut put_down,
                                &mut player,
                                k,
                                player_discard,
                                &mut discard,
                                *player_turn,
                                &mut real_score,
                                &mut player_down,
                                player_put_down,
                                n,
                            );
                        }
                        x += 1;
                    }
                }
                if *game_round == 3 {
                    set_counter = 2;
                    run_counter = 1;

                    let mut x = 1;
                    while x <= set_counter {
                        println!("Enter set {}:", x);
                        let mut put_down = card_put_down();
                        if valid_put_down(&mut put_down, set_counter, run_counter) == false {
                            //cards are not a run or a set
                            println!("Sorry, the cards you entered are not valid.")
                        } else {
                            remove_card(
                                &mut put_down,
                                &mut player,
                                k,
                                player_discard,
                                &mut discard,
                                *player_turn,
                                &mut real_score,
                                &mut player_down,
                                player_put_down,
                                n,
                            );
                        }
                        x += 1;
                    }
                    x = 1;
                    while x <= run_counter {
                        println!("Enter run {}:", x);
                        let mut put_down = card_put_down();
                        if valid_put_down(&mut put_down, set_counter, run_counter) == false {
                            //cards are not a run or a set
                            println!("Sorry, the cards you entered are not valid.")
                        } else {
                            remove_card(
                                &mut put_down,
                                &mut player,
                                k,
                                player_discard,
                                &mut discard,
                                *player_turn,
                                &mut real_score,
                                &mut player_down,
                                player_put_down,
                                n,
                            );
                        }
                        x += 1;
                    }
                }
                if *game_round == 4 {
                    set_counter = 1;
                    run_counter = 2;

                    let mut x = 1;
                    while x <= set_counter {
                        println!("Enter set {}:", x);
                        let mut put_down = card_put_down();
                        if valid_put_down(&mut put_down, set_counter, run_counter) == false {
                            //cards are not a run or a set
                            println!("Sorry, the cards you entered are not valid.")
                        } else {
                            remove_card(
                                &mut put_down,
                                &mut player,
                                k,
                                player_discard,
                                &mut discard,
                                *player_turn,
                                &mut real_score,
                                &mut player_down,
                                player_put_down,
                                n,
                            );
                        }
                        x += 1;
                    }
                    x = 1;
                    while x <= run_counter {
                        println!("Enter run {}:", x);
                        let mut put_down = card_put_down();
                        if valid_put_down(&mut put_down, set_counter, run_counter) == false {
                            //cards are not a run or a set
                            println!("Sorry, the cards you entered are not valid.")
                        } else {
                            remove_card(
                                &mut put_down,
                                &mut player,
                                k,
                                player_discard,
                                &mut discard,
                                *player_turn,
                                &mut real_score,
                                &mut player_down,
                                player_put_down,
                                n,
                            );
                        }
                        x += 1;
                    }
                }
                if *game_round == 5 {
                    set_counter = 3;
                    run_counter = -1;

                    let mut x = 1;
                    while x <= set_counter {
                        println!("Enter set {}:", x);
                        let mut put_down = card_put_down();
                        if valid_put_down(&mut put_down, set_counter, run_counter) == false {
                            //cards are not a run or a set
                            println!("Sorry, the cards you entered are not valid.")
                        } else {
                            remove_card(
                                &mut put_down,
                                &mut player,
                                k,
                                player_discard,
                                &mut discard,
                                *player_turn,
                                &mut real_score,
                                &mut player_down,
                                player_put_down,
                                n,
                            );
                        }
                        x += 1;
                    }
                }
                if *game_round == 6 {
                    run_counter = 3;
                    set_counter = -1;

                    let mut x = 1;
                    while x <= run_counter {
                        println!("Enter run {}:", x);
                        let mut put_down = card_put_down();
                        if valid_put_down(&mut put_down, set_counter, run_counter) == false {
                            //cards are not a run or a set
                            println!("Sorry, the cards you entered are not valid.")
                        } else {
                            remove_card(
                                &mut put_down,
                                &mut player,
                                k,
                                player_discard,
                                &mut discard,
                                *player_turn,
                                &mut real_score,
                                &mut player_down,
                                player_put_down,
                                n,
                            );
                        }

                        x += 1;
                    }
                }
            }
            
            println!("Player {} Hand: {}", player_turn, player[k]);

        }
        if n == 5 {
            player[k].sort_suit_ascending_rank();
        }
        if n == 6 {
            player[k].sort_suit_descending_rank();
        }

        if n == 7 {
            let mut temp_k = 0;
            for mut x in 0..3 {
                x += 1;
                println!("Player {} Score: {}", x, real_score[temp_k]);
                //real_score[temp_k] = score[temp_k];
                temp_k += 1;
                x -= 1;
            }
            if player[k].len() == 0 {
                player_out = true;
                deck.reset_shuffle();
            }
        }
        if *player_turn == 4 {
            *player_turn = 1;
        }
        if n == 9 {
            rules();
        }
        if n == 0 {
            player_out = true;
            deck.reset_shuffle();
        }
        deck.reset_shuffle();
    }
}

fn card_put_down() -> Hand {
    //code taken from online            slightly modified but using this answer https://stackoverflow.com/a/26537398/13119234
    use std::io::BufRead; // (a)
    let reader = io::stdin();
    let the_card: Vec<String> = reader //changed from "numbers" to "the_card" and changed vec to a string
        .lock() // (0)
        .lines()
        .next()
        .unwrap()
        .unwrap() // (1)
        .split(' ')
        .map(|s| s.trim()) // (2)
        .filter(|s| !s.is_empty()) // (3)
        .map(|s| s.parse().unwrap()) // (4)
        .collect(); // (5)
                    //println!("{:?}", the_card);
                    //end of slightly modified but using this answer https://stackoverflow.com/a/26537398/13119234
    println!("-----------------------------");
    let mut temp_put_down = Hand::new(); //puts the card the player is putting down into a temp hand
    let mut put_down = Hand::new(); //this hand compares the players hand with the cards they wanna put down

    for y in the_card {
        //println!(" c/s {}", y);
        let s2 = &y;

        temp_put_down = Hand::from_strings(&[s2]);
        put_down.push_hand(&temp_put_down);
    }
    return put_down;
    //valid_put_down(&mut put_down);
}
fn valid_put_down(put_down: &mut Hand, set_counter: i32, run_counter: i32) -> bool {
    let mut h = 0;
    let mut j = 1;

    let mut set = false;
    let mut x = 1;
    while x <= set_counter {
        while j < put_down.len() {
            //set
            let mut c1 = put_down.cards[j];
            let mut c2 = put_down.cards[j - 1];
            if c1.rank.ordinal() != c2.rank.ordinal() && (c1.rank.ordinal() != 50) && (c2.rank.ordinal() != 50)
            //false
            {
                h = 1;
            } else
            //true
            {
                h = 0;
            }
            j += 1;
        }
        x += 1;
    }
    x = 1;
    while x <= run_counter {
        while j < put_down.len() {
            //run
            let mut c1 = put_down.cards[j];
            let mut c2 = put_down.cards[j - 1];
            if (c1.rank.ordinal() != c2.rank.ordinal() + 1)
                && (c1.suit.ordinal() == c2.suit.ordinal() + 1) && (c1.rank.ordinal() != 50) && (c2.rank.ordinal() != 50)
            //false
            {
                h = 1;
            } else
            //true
            {
                h = 0;
            }
            j += 1;
        }
        x += 1;
    }
    if h == 1 {
        return false;
    } else {
        return true;
    }
}

fn remove_card(
    put_down: &mut Hand,
    player: &mut [Hand; 3],
    k: usize,
    mut player_discard: bool,
    mut discard: &mut Hand,
    mut player_turn: i64,
    real_score: &mut [i64; 3],
    player_down: &mut [Hand; 3],
    mut player_put_down: bool,
    mut n: i32,
) {
    let mut h = 0;
    while h != put_down.len() {
        let mut x = 0;
        while x != (player[k].len()) {
            //println!("card being done {}", player[k]);
            //println!("card being searched for {}", put_down.cards[h]);
            let mut c1 = player[k].cards[x];
            //println!("test {}", c1.rank.ordinal());

            c1.to_string();

            //println!("Player {} Hand lens: {}", player_turn, player[k].len());
            //println!("h value is {}", h);
            //println!("x value is {}", x);
            if player[k].cards[x] == put_down.cards[h] {
                player[k].remove_card(&c1);
                if player_discard == true {
                    player_turn += 1;
                    discard.push_card(c1); //removes a card from the hand and then puts it in the discard pile
                }else {
                    player_down[k].push_card(c1);
                    player_discard = false;
                    player_put_down = true;
                }
                if player[k].len() == 0 {
                    println!("Player is out!!!");
                    println!("-----------------------------");
                    for p in 0..3 {
                        let mut h = 0;
                        while h != player[p].len() {
                            let mut c2 = player[p].cards[h];

                            if c2.rank.ordinal() <= 7 {
                                real_score[p] += 5;
                                //println!("score: {}", score[k]);
                            }

                            if c2.rank.ordinal() > 7 && c2.rank.ordinal() <= 11 {
                                real_score[p] += 10;
                                //println!("score: {}", score[k]);
                            }

                            if c2.rank.ordinal() == 12 {
                                real_score[p] += 15;
                                //println!("score: {}", score[k]);
                            }
                            if c2.rank.ordinal() == 50 {
                                real_score[p] += 50;
                            }
                            h += 1;
                        }
                    }
                    //n = 7;
                } 
                //println!("Player {} Hand if: {}", player_turn, player[k]);
                h += 1;
                x = 0;
            } else {
                x += 1;
            }
            if h == put_down.len() {
                break;
            }
        }
        //println!("Player {} Hand: {}", player_turn, player[k]);
    }
}

fn round_1(real_score: &mut [i64; 3], player_turn: &mut i64, game_round: &mut i64) {
    println!("-----------------------------");
    println!("This hand is two sets.");
    println!("-----------------------------");

    let mut pass_real_score = real_score;
    let mut pass_player_turn = player_turn;
    let mut pass_game_round = game_round;
    deal_cards(&mut pass_real_score, &mut pass_player_turn, &mut pass_game_round);
}

fn round_2(real_score: &mut [i64; 3], player_turn: &mut i64, game_round: &mut i64) {
    println!("-----------------------------");
    println!("This hand is two runs.");
    println!("-----------------------------");
    let mut pass_real_score = real_score;
    let mut pass_player_turn = player_turn;
    let mut pass_game_round = game_round;
    deal_cards(&mut pass_real_score, &mut pass_player_turn, &mut pass_game_round);
}

fn round_3(real_score: &mut [i64; 3], player_turn: &mut i64, game_round: &mut i64) {
    println!("-----------------------------");
    println!("This hand is two sets and a run.");
    println!("-----------------------------");
    let mut pass_real_score = real_score;
    let mut pass_player_turn = player_turn;
    let mut pass_game_round = game_round;
    deal_cards(&mut pass_real_score, &mut pass_player_turn, &mut pass_game_round);
}

fn round_4(real_score: &mut [i64; 3], player_turn: &mut i64, game_round: &mut i64) {
    println!("-----------------------------");
    println!("This hand is two runs and a set.");
    println!("-----------------------------");
    let mut pass_real_score = real_score;
    let mut pass_player_turn = player_turn;
    let mut pass_game_round = game_round;
    deal_cards(&mut pass_real_score, &mut pass_player_turn, &mut pass_game_round);
}

fn round_5(real_score: &mut [i64; 3], player_turn: &mut i64, game_round: &mut i64) {
    println!("-----------------------------");
    println!("This hand is 3 sets.");
    println!("-----------------------------");
    let mut pass_real_score = real_score;
    let mut pass_player_turn = player_turn;
    let mut pass_game_round = game_round;
    deal_cards(&mut pass_real_score, &mut pass_player_turn, &mut pass_game_round);
}

fn round_6(real_score: &mut [i64; 3], player_turn: &mut i64, game_round: &mut i64) {
    println!("-----------------------------");
    println!("This hand is 3 runs.");
    println!("-----------------------------");
    let mut pass_real_score = real_score;
    let mut pass_player_turn = player_turn;
    let mut pass_game_round = game_round;
    deal_cards(&mut pass_real_score, &mut pass_player_turn, &mut pass_game_round);
}

fn rules() {
    let mut x = 1;
    while x != 0 {
        println!("------------Rules------------");
        println!("1.\t");
        println!("2.\t");
        println!("3.\t");
        println!("4.\t");
        println!("5.\t");
        println!("6.\t");
        println!("7.\t");
        println!("0.\t Exit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut n: i32 = input.trim().parse().unwrap();
        x = n;
    }
}
fn main() {
    let mut game_round: i64 = 1;
    let mut real_score: [i64; 3] = [0, 0, 0];
    let mut player_turn: i64 = 1;

    if game_round == 1 {
        round_1(&mut real_score, &mut player_turn, &mut game_round);
        game_round = 2;
    }
    if game_round == 2 {
        round_2(&mut real_score, &mut player_turn, &mut game_round);
        game_round = 3;
    }
    if game_round == 3 {
        round_3(&mut real_score, &mut player_turn, &mut game_round);
        game_round = 4;
    }
    if game_round == 4 {
        round_4(&mut real_score, &mut player_turn, &mut game_round);
        game_round = 5;
    }
    if game_round == 5 {
        round_5(&mut real_score, &mut player_turn, &mut game_round);
        game_round = 6;
    }
    if game_round == 6 {
        round_6(&mut real_score, &mut player_turn, &mut game_round);
    }
}
