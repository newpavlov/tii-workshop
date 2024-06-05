#[derive(Debug, PartialEq)]
pub struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

trait Balanceable {
    fn get_balance(self: &Self) -> i64;
    fn set_balance(self: &mut Self, value: i64);
}

impl Balanceable for User {
    fn get_balance(self: &Self) -> i64 {
        self.balance
    }

    fn set_balance(self: &mut Self, value: i64) {
        self.balance = value
    }
}

impl User {
    // type of name?
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self { name, credit_line, balance }
    }
}

impl Balance for User {
    fn balance(self: &Self) -> i64 {
        self.balance
    }
}

trait Balance {
    fn balance(self: &Self) -> i64;
}

#[derive(Debug, PartialEq)]
pub struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Balance for Bank {
    fn balance(self: &Self) -> i64 {
        return self.users.iter().map(|user| user.balance()).sum();
    }
}

impl Bank {
    // type of users?
    pub fn new(users: Vec<User>, name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self { users, name, credit_interest, debit_interest }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_a_user() {
        let user = User::new("John".to_string(), 1, 1);

        assert_eq!(user.name, "John".to_string());
        assert_eq!(user.credit_line, 1);
        assert_eq!(user.balance, 1);
    }

    #[test]
    fn test_create_a_bank() {
        let user1 = User::new("John".to_string(), 1, 1);
        let bank = Bank::new(vec![user1], "First Bank".to_string(), 1, 4);


        assert_eq!(bank.users.len(), 1);
        assert_eq!(bank.name, "First Bank".to_string());
        assert_eq!(bank.credit_interest, 1);
        assert_eq!(bank.debit_interest, 4);
    }

    #[test]
    fn test_equality_for_bank() {
        let user1 = User::new("John".to_string(), 1, 1);
        let user2 = User::new("John".to_string(), 1, 1);
        let bank = Bank::new(vec![user1], "First Bank".to_string(), 1, 4);
        let bank2 = Bank::new(vec![user2], "First Bank".to_string(), 1, 4);

        assert_eq!(bank, bank2);
    }

    // - calc_balance: calculates bank’s balance sheet in the form of two numbers: total bank liabilities and assets (liabilities represent all debit accounts, assets represent all credit accounts).
    // – transfer_funds: accepts two user names and transfer amount as positive integer. Transfers the specified amount from one user to another. Returns an error, if its can not be done (e.g. if the origin user hit his credit limit).
    // – accrue_interest: update user balances according to bank’s interest rates on credit and debit.
    #[test]
    fn calculate_balance_of_the_user() {
        let user1 = User::new("John".to_string(), 100, 1);

        let total_balance = user1.balance();

        assert_eq!(total_balance, 1);
    }

    #[test]
    fn calculate_balance_of_the_bank() {
        let user1 = User::new("John".to_string(), 100, 1);
        let user2 = User::new("John".to_string(), 1, 90);
        let bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let total_balance = bank.balance();

        assert_eq!(total_balance, 1 + 90);
    }
}