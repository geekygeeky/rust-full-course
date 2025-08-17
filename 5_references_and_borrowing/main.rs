// References allow you to borrow values without taking ownership of the variable

fn main() {

    let mut x = 5;
    let age = x * 2;
    let r = &mut x;

    *r += 1;

    println!("Value of r: {}", r);
    println!("Value of x: {}", x);
    println!("Value of x: {}", age);

    let mut account = BankAccount{
        owner: "Joe Schmoe".to_string(),
        balance: 200.30
    };

    account.check_balance();

    account.withdraw(20.0);

    account.check_balance();

}

struct BankAccount{
    owner: String,
    balance: f64
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance)
    }
}