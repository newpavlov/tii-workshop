#[derive(Debug, PartialEq)]
pub struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

trait BalanceAccessor {
    fn get_balance(&self) -> i64;
    fn set_balance(&mut self, value: i64);
}

impl BalanceAccessor for User {
    fn get_balance(&self) -> i64 {
        self.balance
    }

    fn set_balance(&mut self, value: i64) {
        self.balance = value
    }
}

impl User {
    // type of name?
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self { name, credit_line, balance }
    }
}

impl MaxCredit for User {
    fn max_credit(&self) -> u64 {
        self.balance as u64 + self.credit_line
    }
}

trait Balance {
    fn balance(&self) -> BalanceSheet;
}

trait MaxCredit {
    fn max_credit(&self) -> u64;
}

#[derive(Debug, PartialEq)]
pub struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {}

pub struct BalanceSheet {
    liabilities: u64,
    assets: i64,
}

impl Balance for Bank {
    fn balance(&self) -> BalanceSheet {
        let total_assets = self.users.iter().map(|user| user.get_balance()).sum();
        let total_liabilities = self.users.iter().map(|user| user.credit_line).sum();
        return BalanceSheet { assets: total_assets, liabilities: total_liabilities };
    }
}

impl Bank {
    // type of users?
    pub fn new(users: Vec<User>, name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self { users, name, credit_interest, debit_interest }
    }
    pub(crate) fn get_user_by_id(&self, user_id: String) -> Option<&User> {
        for user in self.users.iter() {
            if user.name == user_id {
                return Some(&user);
            }
        }
        return None;
    }
    pub(crate) fn transfer(&mut self, origin_username: &str, destination_username: &str, amount: i64) -> bool {
        if amount <= 0 {
            return false;
        }
        let mut maybe_origin_index = None;
        let mut maybe_destination_index = None;
        // let users_iter_mut = self.users.iter_mut();
        // let both_users_exist = users_iter_mut.filter(|user| user.name == origin_username || user.name == destination_username).count() == 2;
        for (index, user) in self.users.iter().enumerate() {
            if user.name == origin_username {
                maybe_origin_index = Some(index);
            }
            if user.name == destination_username {
                maybe_destination_index = Some(index);
            }
            if maybe_origin_index.is_some() && maybe_destination_index.is_some() {
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

        let (origin_index, destination_index) = match (maybe_origin_index, maybe_destination_index) {
            (Some(origin_index), Some(destination_index)) => { (origin_index, destination_index) }
            (None, _) => { return false; }
            (_, None) => { return false; }
        };

        let has_credit_limit = self.users[origin_index].max_credit() >= amount as u64;
        if !has_credit_limit {
            return false;
        }


        {
            let origin_balance = self.users[origin_index].get_balance() - amount;
            self.users[origin_index].set_balance(origin_balance);
        }
        {
            let destination_balance = self.users[destination_index].get_balance() + amount;
            self.users[destination_index].set_balance(destination_balance);
        }

        // self.users[first_user_index.unwrap()]
        // second_user.map(|mut user| user.set_balance(user.get_balance()));

        true
    }

    pub(crate) fn accrue_interest(&mut self) -> bool {
        for user in self.users.iter_mut() {
            {
                let debit_interest = ((user.balance as u64 * self.debit_interest) / 100) as i64;
                let new_balance = user.balance + debit_interest;
                user.set_balance(new_balance);
            }

            {
                let credit_interest = (user.credit_line * self.credit_interest) / 100;
                let new_credit_line = user.credit_line + credit_interest;
                user.credit_line = new_credit_line;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests_user {
    use super::*;

    #[test]
    fn assign_the_fields() {
        let user = User::new("John".to_string(), 1, 1);

        assert_eq!(user.name, "John".to_string());
        assert_eq!(user.credit_line, 1);
        assert_eq!(user.balance, 1);
    }

    #[test]
    fn equality_of_the_object_includes_equality_of_all_fields() {
        let user1 = User::new("John".to_string(), 1, 1);
        let user2 = User::new("John".to_string(), 1, 1);

        assert_eq!(user1, user2);
    }
}

#[cfg(test)]
mod tests_bank {
    use super::*;

    #[test]
    fn assign_the_fields() {
        let user1 = User::new("John".to_string(), 1, 1);
        let bank = Bank::new(vec![user1], "First Bank".to_string(), 1, 4);


        assert_eq!(bank.users.len(), 1);
        assert_eq!(bank.name, "First Bank".to_string());
        assert_eq!(bank.credit_interest, 1);
        assert_eq!(bank.debit_interest, 4);
    }

    #[test]
    fn equality_of_the_object_includes_equality_of_all_fields() {
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
    fn calculate_balance_of_the_bank() {
        let user1 = User::new("John1".to_string(), 100, 1);
        let user2 = User::new("John".to_string(), 1, 90);
        let bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let total_balance = bank.balance();

        assert_eq!(total_balance.assets, 1 + 90);
        assert_eq!(total_balance.liabilities, 100 + 1);
    }

    #[test]
    fn transfer_funds_happy_path() {
        let user1 = User::new("user1".to_string(), 100, 1);
        let user2 = User::new("user2".to_string(), 1, 90);
        let mut bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer("user1", "user2", 2);

        assert_eq!(result, true);
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().balance, -1);
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance, 92);
    }

    #[test]
    fn transfer_funds_credit_limit_exceeded() {
        let user1 = User::new("user1".to_string(), 0, 1);
        let user2 = User::new("user2".to_string(), 1, 90);
        let mut bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer("user1", "user2", 2);

        assert_eq!(result, false);
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().balance, 1);
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance, 90);
    }

    #[test]
    fn transfer_funds_origin_user_does_not_exist() {
        let user2 = User::new("user2".to_string(), 0, 1);
        let mut bank = Bank::new(vec![user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer("NON_EXISTING", "user2", 2);

        assert_eq!(result, false);
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance, 1);
    }

    #[test]
    fn transfer_funds_destination_user_does_not_exist() {
        let user2 = User::new("user2".to_string(), 0, 1);
        let mut bank = Bank::new(vec![user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer("user2", "NON_EXISTING", 2);

        assert_eq!(result, false);
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance, 1);
    }

    #[test]
    fn transfer_funds_amount_cannot_be_negative() {
        let user1 = User::new("user1".to_string(), 10, 1);
        let user2 = User::new("user2".to_string(), 1, 90);
        let mut bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer("user1", "user2", -2);

        assert_eq!(result, false);
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().balance, 1);
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance, 90);
    }


    #[test]
    fn transfer_funds_amount_cannot_be_zero() {
        let user1 = User::new("user1".to_string(), 10, 1);
        let user2 = User::new("user2".to_string(), 1, 90);
        let mut bank = Bank::new(vec![user1, user2], "First Bank".to_string(), 1, 4);

        let result = bank.transfer("user1", "user2", 0);

        assert_eq!(result, false);
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().balance, 1);
        assert_eq!(bank.get_user_by_id("user2".to_string()).unwrap().balance, 90);
    }

    #[test]
    fn bank_accrue_interest_for_debits() {
        let user1 = User::new("user1".to_string(), 0, 100);
        let mut bank = Bank::new(vec![user1], "First Bank".to_string(), 1, 4);

        let result = bank.accrue_interest();

        assert_eq!(result, true);
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().balance, 104);
    }

    #[test]
    fn bank_accrue_interest_for_credits() {
        let user1 = User::new("user1".to_string(), 100, 0);
        let mut bank = Bank::new(vec![user1], "First Bank".to_string(), 1, 4);

        let result = bank.accrue_interest();

        assert_eq!(result, true);
        assert_eq!(bank.get_user_by_id("user1".to_string()).unwrap().credit_line, 101);
    }
}