pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut maximum_wealth = 0;
    accounts
        .iter()
        .for_each(|customer| {
            let customer_wealth = customer.iter().sum(); 
            if maximum_wealth < customer_wealth {
                maximum_wealth = customer_wealth; 
            }
        });
    return maximum_wealth;
}
