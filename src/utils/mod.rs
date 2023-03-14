use crate::currencies::CurrencyKind;
use crate::currencies::Money;

pub fn calculate_total(tender: &[&Money]) -> i64 {
    // Calculates the total value of a vector of tender.
    let mut total: i64 = 0;
    for money in tender {
        total += money.value
    }
    total
}

pub fn all_money_same_currency(tender: &[&Money]) -> bool {
    // We could propagate the error upwards to see what the caller of this function wants to do with this error.
    match tender.is_empty() {
        true => panic!("There were no tender to compare."),
        false => (),
    }
    let first_coin = tender.iter().next();

    let first_currency: &CurrencyKind = match first_coin {
        Some(coin) => &coin.currency,
        None => panic!("Something went wrong. There was an error counting your tender."),
    };
    for coin in tender {
        if &coin.currency != first_currency {
            return false;
        }
    }
    true
}

fn change_calculating_algorithm(
    change_vector: &mut Vec<&Money>,
    change: i64,
    currency: CurrencyKind,
) -> i64 {
    let available_tender = currency.get_tender();
    let mut change = change.to_owned();
    // available_tender.iter().rev() because we want to get the highest value tender first.
    // available_tender is ordered from low to high value when it is defined.
    for coin in available_tender.iter().rev() {
        if change >= coin.value {
            change_vector.push(coin);
            change -= coin.value;
            break;
        }
    }
    change
}

pub fn calculate_change(tender_paid: &[&Money], price: i64) -> Vec<&'static Money> {
    if let false = all_money_same_currency(tender_paid) {
        panic!("Sorry, something went wrong. All tender must be the same currency.");
    }
    let total: i64 = calculate_total(tender_paid);
    let mut change_vector: Vec<&Money> = vec![];
    let mut change = total - price;

    let currency = match tender_paid.iter().next() {
        Some(coin) => &coin.currency,
        None => panic!("Sorry, something went wrong. No money found in tender paid."),
    };

    if change < 0 {
        panic!("Sorry, you have not paid enough.");
    }
    if change == 0 {
        println!("Great! You have paid the exact amount.");
    }
    if change > 0 {
        while change > 0 {
            change = change_calculating_algorithm(&mut change_vector, change, *currency)
        }
    }
    change_vector
}
