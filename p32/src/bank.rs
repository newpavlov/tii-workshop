use std::ptr::null;

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

    fn max_credit(self: &Self) -> u64 {
        self.balance as u64 + self.credit_line
    }
}

trait Balance {
    fn balance(self: &Self) -> i64;
    fn max_credit(self: &Self) -> u64;
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

    fn max_credit(self: &Self) -> u64 {
        return self.users.iter().map(|user| user.max_credit()).sum();
    }
}

impl Bank {
    // type of users?
    pub fn new(users: Vec<User>, name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self { users, name, credit_interest, debit_interest }
    }
    pub(crate) fn get_user_by_id(&self, user_id: String) -> Option<&User> {
        for (index, user) in self.users.iter().enumerate() {
            if user.name == user_id {
                return Some(&user);
            }
        }
        return None;
    }
    pub(crate) fn transfer(&mut self, origin_username: &str, destination_username: &str, amount: i64) -> bool {
        let mut first_user_index = None;
        let mut second_user_index = None;
        // let users_iter_mut = self.users.iter_mut();
        // let both_users_exist = users_iter_mut.filter(|user| user.name == origin_username || user.name == destination_username).count() == 2;
        for (index, user) in self.users.iter().enumerate() {
            if user.name == origin_username {
                first_user_index = Some(index);
            }
            if user.name == destination_username {
                second_user_index = Some(index);
            }
            if first_user_index.is_some() && second_user_index.is_some() {
                break;
            }
        }


        // let both_users_exist = users_iter_mut.filter(|user| user.name == origin_username || user.name == destination_username).count() == 2;
        // first_user_exists = users_iter_mut(|user| user.name == origin_username).count() == 1;
        // let users_iter_mut = self.users.iter_mut();
        // let second_user_exists = users_iter_mut.filter(|user| user.name == destination_username).count() == 1;

        // let mut first_user = self.users[first_user_exists_index as usize];
        // first_user.set_balance(first_user.get_balance() - amount);
        // self.users[second_user_exists_index as usize].set_balance(self.users[second_user_exists_index as usize].get_balance() + amount);

        let (first_index, second_index) = match (first_user_index, second_user_index) {
            (Some(first_index), Some(second_index)) => { (first_index, second_index) }
            (_, Some(_)) => { return false; }
            _ => todo!()
        };

        let has_credit_limit = self.users[first_index].max_credit() >= amount as u64;
        if !has_credit_limit {
            return false;
        }


        {
            let new_balance = self.users[first_index].get_balance() - amount;
            self.users[first_index].set_balance(new_balance);
        }
        {
            let new_balance_2 = self.users[second_index].get_balance() + amount;
            self.users[second_index].set_balance(new_balance_2);
        }

        // self.users[first_user_index.unwrap()]
        // second_user.map(|mut user| user.set_balance(user.get_balance()));

        true
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
        let user1 = User::new("John1".to_string(), 100, 1);
        let user2 = User::new("John".to_string(), 1, 90);
        let bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let total_balance = bank.balance();

        assert_eq!(total_balance, 1 + 90);
    }

    #[test]
    fn transfer_funds_happy_path() {
        let user1 = User::new("user1".to_string(), 100, 1);
        let user2 = User::new("user2".to_string(), 1, 90);
        let mut bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer("user1", "user2", 2);

        assert_eq!(result, true);
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().balance(), -1);
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance(), 92);
    }

    #[test]
    fn transfer_funds_credit_limit_exceeded() {
        let user1 = User::new("user1".to_string(), 0, 1);
        let user2 = User::new("user2".to_string(), 1, 90);
        let mut bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer("user1", "user2", 2);

        assert_eq!(result, false);
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().balance(), 1);
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance(), 90);
    }

    #[test]
    fn transfer_funds_origin_user_does_not_exist() {
        let user2 = User::new("user2".to_string(), 0, 1);
        let mut bank = Bank::new(vec![user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer("NON_EXISTING", "user2", 2);

        assert_eq!(result, false);
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance(), 1);
    }
}