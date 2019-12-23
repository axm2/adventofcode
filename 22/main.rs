use std::collections::VecDeque;

const DECKSIZE: usize = 10007;

fn main() {
    let mut deck = VecDeque::new();
    for x in 0..DECKSIZE{
        deck.push_back(x);
    }
        deck = cut(deck, 9374);
        deck = deal(deck, 48);
        deck = cut(deck, -2354);
        deck = deal(deck, 12);
        deck = cut(deck, -7039);
        deck = deal(deck, 14);
        deck = cut(deck, -2325);
        deck = deal(deck, 40);
        deck = new_stack(deck);
        deck = cut(deck, 4219);
        deck = deal(deck, 15);
        deck = cut(deck, -3393);
        deck = deal(deck, 48);
        deck = cut(deck, 1221);
        deck = deal(deck, 66);
        deck = cut(deck, 1336);
        deck = deal(deck, 53);
        deck = new_stack(deck);
        deck = cut(deck, -5008);
        deck = new_stack(deck);
        deck = deal(deck, 34);
        deck = cut(deck, 8509);
        deck = deal(deck, 24);
        deck = cut(deck, -1292);
        deck = new_stack(deck);
        deck = cut(deck, 8404);
        deck = deal(deck, 17);
        deck = cut(deck, -105);
        deck = deal(deck, 51);
        deck = cut(deck, 2974);
        deck = deal(deck, 5);
        deck = new_stack(deck);
        deck = deal(deck, 53);
        deck = cut(deck, 155);
        deck = deal(deck, 31);
        deck = cut(deck, 2831);
        deck = deal(deck, 61);
        deck = cut(deck, -4193);
        deck = new_stack(deck);
        deck = cut(deck, 9942);
        deck = deal(deck, 13);
        deck = cut(deck, -532);
        deck = deal(deck, 41);
        deck = cut(deck, 2847);
        deck = new_stack(deck);
        deck = cut(deck, -2609);
        deck = deal(deck, 72);
        deck = cut(deck, 9098);
        deck = deal(deck, 64);
        deck = new_stack(deck);
        deck = cut(deck, 4292);
        deck = new_stack(deck);
        deck = cut(deck, -4427);
        deck = deal(deck, 24);
        deck = cut(deck, -4713);
        deck = new_stack(deck);
        deck = cut(deck, 5898);
        deck = deal(deck, 56);
        deck = cut(deck, -2515);
        deck = deal(deck, 2);
        deck = cut(deck, -5502);
        deck = deal(deck, 66);
        deck = cut(deck, 8414);
        deck = deal(deck, 7);
        deck = new_stack(deck);
        deck = deal(deck, 35);
        deck = new_stack(deck);
        deck = deal(deck, 29);
        deck = cut(deck, -2176);
        deck = deal(deck, 14);
        deck = cut(deck, 7773);
        deck = deal(deck, 36);
        deck = cut(deck, 2903);
        deck = new_stack(deck);
        deck = deal(deck, 75);
        deck = cut(deck, 239);
        deck = deal(deck, 45);
        deck = cut(deck, 5450);
        deck = deal(deck, 10);
        deck = cut(deck, 6661);
        deck = deal(deck, 64);
        deck = cut(deck, -6842);
        deck = deal(deck, 40);
        deck = new_stack(deck);
        deck = deal(deck, 31);
        deck = new_stack(deck);
        deck = deal(deck, 46);
        deck = cut(deck, 6462);
        deck = new_stack(deck);
        deck = cut(deck, -8752);
        deck = deal(deck, 28);
        deck = new_stack(deck);
        deck = deal(deck, 43);
        deck = new_stack(deck);
        deck = deal(deck, 54);
        deck = cut(deck, 9645);
        deck = deal(deck, 44);
        deck = cut(deck, 5342);
        deck = deal(deck, 66);
        deck = cut(deck, 3785);
        find(deck, 2019);

}


fn new_stack(deck: VecDeque<usize>)->VecDeque<usize>{
    let mut i=0;
    let mut j=DECKSIZE-1;
    let mut tempdeck = deck;
    while i!=DECKSIZE/2{
        tempdeck.swap(i,j);
        i=i+1;
        j=j-1;
    }
    return tempdeck;
}

fn cut(deck: VecDeque<usize>, n: i32)->VecDeque<usize>{ // Shouldve used rotate_left and rotate_right
    let mut tempdeck = deck;
    let mut result = VecDeque::new();
    if n<0{
        let tnum = n.abs() as usize;
        let num = DECKSIZE - tnum;
        let mut tempdeck2 = tempdeck.split_off(num);
        result.append(&mut tempdeck2);
        result.append(&mut tempdeck);
        return result;
    }
    else{
        let num = n as usize;
        let mut tempdeck2 = tempdeck.split_off(num);
        result.append(&mut tempdeck2);
        result.append(&mut tempdeck);
        return result;
    }

}

fn deal(deck:VecDeque<usize>, n: usize)->VecDeque<usize>{
    let mut result = deck.clone();
    let mut tempdeck = deck.clone();
    let mut j = 0;
    for i in 0..DECKSIZE{
        if j>DECKSIZE{
            j=j-DECKSIZE;
        }
        let mut f = tempdeck.pop_front();
        if let Some(elem) = result.get_mut(j) {
            *elem = f.unwrap();
            j=j+n
        }

    }
    return result;
}

fn find(deck:VecDeque<usize>, n: usize){
    for i in 0..DECKSIZE{
        let mut num = *deck.get(i).unwrap();
        if num == n{
            print!("{}",i);
        }
    }
}




