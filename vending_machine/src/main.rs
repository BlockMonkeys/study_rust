#[derive(Debug)]
struct VendingMachine {
    money : u32,
    products : Vec<Beverage>
}
#[derive(Debug)]
struct Beverage {
    name : String,
    price : u32,
    amount : u32
}

impl VendingMachine {
    fn buy(mut self, pName : String, inMoney : u32) {
        let result = self.products;
        let mut moneyBalance = inMoney;
        let mut flag = false;
        for mut i in result {
            if i.name == pName {
                if moneyBalance < i.price {
                    println!("잔고가 부족합니다.");
                } else {
                    //물건을 팔아라.
                    i.amount = i.amount - 1;
                    moneyBalance = moneyBalance - i.price;
                    flag = true;
                }
            }
        }

        if moneyBalance > 0 {
            self.money -= moneyBalance;
            println!("잔돈반환 : {}, 머신 남은돈 : {}", moneyBalance, &self.money);
        }
    }
}


fn main() {
    let v : Vec<Beverage> = vec![];
    let mut myVM = VendingMachine {
        money : 10_000,
        products : v
    };

    let cola : Beverage = Beverage {
        name : String::from("Cola"),
        price : 1500,
        amount : 2
    };

    let milkis : Beverage = Beverage {
        name : String::from("Milkis"),
        price : 1800,
        amount : 2
    };

    myVM.products.push(cola);
    myVM.products.push(milkis);

    myVM.buy(String::from("Milkis"), 2000);
}
