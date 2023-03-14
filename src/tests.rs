#[cfg(test)]
mod tests {
    use crate::currencies::CurrencyKind;
    use crate::currencies::Money;
    use crate::utils::all_money_same_currency;
    use crate::utils::calculate_change;
    #[test]
    fn test_all_moneys_same_currency_function() {
        let two_pence: Money = Money {
            value: 2,
            currency: CurrencyKind::Gbp,
        };

        let one_euro: Money = Money {
            value: 1,
            currency: CurrencyKind::Eur,
        };

        let one_dollar_money: Money = Money {
            value: 1,
            currency: CurrencyKind::Usd,
        };

        let non_matching_moneys = [&two_pence, &one_dollar_money, &one_euro];

        assert_eq!(all_money_same_currency(&non_matching_moneys), false);

        let matching_moneys = [&two_pence, &two_pence, &two_pence, &two_pence];

        assert_eq!(all_money_same_currency(&matching_moneys), true);
    }

    #[test]
    fn test_change_calculate_function_gives_correct_change() {
        let one_pence: Money = Money {
            // 1p coin
            value: 1,
            currency: CurrencyKind::Gbp,
        };

        let one_pound: Money = Money {
            // £1 coin
            value: 100,
            currency: CurrencyKind::Gbp,
        };

        let two_pound: Money = Money {
            // £2 coin
            value: 200,
            currency: CurrencyKind::Gbp,
        };

        let ten_pounds = [
            // in £1 coins
            &one_pound, &one_pound, &one_pound, &one_pound, &one_pound, &one_pound, &one_pound,
            &one_pound, &one_pound, &one_pound,
        ];

        let five_pound_note = Money {
            // £5 note
            value: 500,
            currency: CurrencyKind::Gbp,
        };

        assert_eq!(
            calculate_change(&ten_pounds, 299,),
            vec![&five_pound_note, &two_pound, &one_pence]
        );
    }

    #[test]
    #[should_panic]
    fn test_change_calculate_function_panics_if_different_currencies() {
        let one_pound: Money = Money {
            value: 100,
            currency: CurrencyKind::Gbp,
        };

        let one_dollar: Money = Money {
            value: 100,
            currency: CurrencyKind::Usd,
        };

        let mixed_currency = [&one_pound, &one_pound, &one_pound, &one_dollar];

        calculate_change(&mixed_currency, 299);
    }
}
