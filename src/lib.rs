pub mod factorial {
    pub fn factorial(n: u32) -> u32 {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::factorial;

        #[test]
        fn factorial_test() {
            let result = factorial::factorial(5);
            let exp = 120;
            assert_eq!(result, exp);
        }
    }
}

pub mod knapsack {
    use std::cmp::max;

    pub struct Item {
        weight: usize,
        value: usize,
    }

    // // Google Bard wrote this. But it cannot solve the problems...
    // pub fn knapsack(items: &[Item], capacity: usize) -> usize {
    //     let mut dp = vec![0; capacity + 1];
    //     for item in items {
    //         for i in item.weight..capacity + 1 {
    //             dp[i] = std::cmp::max(dp[i], dp[i - item.weight] + item.value);
    //         }
    //     }
    //     return dp[capacity];
    // }

    pub fn knapsack(items: &[Item], capacity: usize) -> usize {
        let result: usize = if items.len() <= 0 {
            0
        } else if capacity < items[0].weight {
            knapsack(&items[1..items.len()], capacity)
        } else {
            max(
                items[0].value + knapsack(&items[1..items.len()], capacity - items[0].weight),
                knapsack(&items[1..items.len()], capacity),
            )
        };

        result
    }

    #[cfg(test)]
    mod tests {
        use crate::knapsack;

        #[test]
        fn knapsack_test() {
            let items = vec![
                knapsack::Item {
                    weight: 1,
                    value: 10,
                },
                knapsack::Item {
                    weight: 2,
                    value: 20,
                },
                knapsack::Item {
                    weight: 3,
                    value: 30,
                },
            ];

            let capacity = 5;

            let max_value = knapsack::knapsack(&items, capacity);
            assert_eq!(max_value, 50);
        }

        #[test]
        fn knapsack_test_for_four_items() {
            let items = vec![
                knapsack::Item {
                    weight: 5,
                    value: 10,
                },
                knapsack::Item {
                    weight: 4,
                    value: 40,
                },
                knapsack::Item {
                    weight: 6,
                    value: 30,
                },
                knapsack::Item {
                    weight: 4,
                    value: 50,
                },
            ];

            let capacity = 10;

            let max_value = knapsack::knapsack(&items, capacity);
            assert_eq!(max_value, 90);
        }
    }
}
