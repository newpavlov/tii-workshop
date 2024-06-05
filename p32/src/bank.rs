pub struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

impl User {
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self { name, credit_line, balance }
    }
}

pub struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl Bank {
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
        let user = Bank::new(vec![user1], "First Bank".to_string(), 1, 4);


        assert_eq!(user.users.len(), 1);
        assert_eq!(user.name, "First Bank".to_string());
        assert_eq!(user.credit_interest, 1);
        assert_eq!(user.debit_interest, 4);
    }
}