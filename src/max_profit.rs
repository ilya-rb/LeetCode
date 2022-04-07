/// You are given an integer array prices where prices[i] is the price of a given stock on the ith day.
///
/// On each day, you may decide to buy and/or sell the stock.
/// You can only hold at most one share of the stock at any time.
/// However, you can buy it then immediately sell it on the same day.
///
/// Find and return the maximum profit you can achieve.
///
/// Input: prices = [7,1,5,3,6,4]
/// Output: 7
/// Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
/// Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
/// Total profit is 4 + 3 = 7.
///
/// Input: prices = [1,2,3,4,5]
/// Output: 4
/// Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
/// Total profit is 4.
///
/// Input: prices = [7,6,4,3,1]
/// Output: 0
/// Explanation: There is no way to make a positive profit,
/// so we never buy the stock to achieve the maximum profit of 0.

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut current_bought_price: Option<i32> = Option::None;

    let calculate_profit_at_price = |bought_price: Option<i32>, current_price: i32| {
        current_price - bought_price.unwrap()
    };

    for (index, &price) in prices.iter().enumerate() {
        match prices.get(index + 1) {
            None => {
                // Sell on the last day if we have any
                if current_bought_price.is_some() {
                    profit += calculate_profit_at_price(current_bought_price, price);
                    current_bought_price = Option::None;
                }
            }
            Some(&next) => {
                if price > next {
                    // Next day is cheaper, sell if we have some
                    if current_bought_price.is_some() {
                        profit += calculate_profit_at_price(current_bought_price, price);
                        current_bought_price = Option::None;
                    }
                } else {
                    // Next day more expensive, let's buy if we don't have any
                    if current_bought_price.is_none() {
                        current_bought_price = Some(price);
                    }
                }
            }
        }
    }

    profit
}

#[cfg(test)]
mod tests {
    use crate::max_profit::max_profit;
    use rstest::rstest;

    #[rstest(prices, expected_profit,
    case(vec ! [7, 1, 5, 3, 6, 4], 7),
    case(vec ! [1, 2, 3, 4, 5], 4),
    case(vec ! [7, 6, 4, 3, 1], 0),
    case(vec ! [2, 1, 2, 0, 1], 2),
    )]
    fn test_max_profit(prices: Vec<i32>, expected_profit: i32) {
        assert_eq!(expected_profit, max_profit(prices));
    }
}