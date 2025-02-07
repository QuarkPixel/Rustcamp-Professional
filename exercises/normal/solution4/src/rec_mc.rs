const COINS: [u32; 8] = [100, 50, 30, 20, 10, 5, 2, 1];

pub fn dp_rec_mc(mut amount: u32) -> u32 {
    let mut coins = COINS.into_iter();
    let mut count = 0;

    while amount != 0 {
        let coin = coins.next().unwrap();
        count += amount / coin;
        amount %= coin;
    }

    count
}