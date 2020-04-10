// https://www.codewars.com/kata/59d727d40e8c9dd2dd00009f/

/*
You are given a (small) check book as a - sometimes - cluttered (by non-alphanumeric characters) string:

"1000.00
125 Market 125.45
126 Hardware 34.95
127 Video 7.45
128 Book 14.32
129 Gasoline 16.10"
The first line shows the original balance. Each other line (when not blank) gives information: check number, category, check amount.

First you have to clean the lines keeping only letters, digits, dots and spaces.

Then return a report as a string (underscores show spaces -- don't put them in your solution. They are there so you can see them and how many of them you need to have):

"Original_Balance:_1000.00
125_Market_125.45_Balance_874.55
126_Hardware_34.95_Balance_839.60
127_Video_7.45_Balance_832.15
128_Book_14.32_Balance_817.83
129_Gasoline_16.10_Balance_801.73
Total_expense__198.27
Average_expense__39.65"
On each line of the report you have to add the new balance and then in the last two lines the total expense and the average expense. So as not to have a too long result string we don't ask for a properly formatted result.
*/

extern crate regex;

fn balance(book: &str) -> String {
println!("book:{:?}", book);
    let re1 = regex::Regex::new(r"[^\n. \dA-Za-z]").unwrap();
    let u = re1.replace_all(book, "");
    let re = regex::Regex::new(r"\n+").unwrap();
    let mut narr: Vec<&str> = re.split(&u).collect::<Vec<&str>>();
    narr.retain(|x| x.len() != 0);
    let total = match narr[0].parse::<f64>() {
        Ok(total) => total, Err(_e) => { -1.0 }
    };
    let mut current = total;
    let mut res = format!("Original Balance: {:.2}", total);
    let mut i = 1;
    while i < narr.len() {
        let l = narr[i].split(" ").collect::<Vec<&str>>();
        let g = match l[2].parse::<f64>() {
            Ok(g) => g, Err(_e) => { -1.0 }
        };
        current -= g;     
        res += &format!("\n{} {} {:.2} Balance {:.2}", l[0], l[1], g, current);
        i += 1;
    }
    res += &format!("\nTotal expense  {:.2}\nAverage expense  {:.2}", 
                    total - current, (total - current) / ((narr.len() -1) as f64));
    return res;
}

Test Cases:
#[cfg(test)]
    extern crate regex as oregex;
    
    mod tests {
    use super::*;
    fn dotest(book: &str, exp: &str) -> () {
        println!("book:{}", book);
        let ans = balance(book);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }
    
    #[test]
    fn basic_tests() {
let b1 = r#"
1000.00!=

125 Market !=:125.45
126 Hardware =34.95
127 Video! 7.45
128 Book :14.32
129 Gasoline ::16.10
"#;
let b2 = r#"
1233.00
125 Hardware;! 24.8?;
123 Flowers 93.5
127 Meat 120.90
120 Picture 34.00
124 Gasoline 11.00
123 Photos;! 71.4?;
122 Picture 93.5
132 Tyres;! 19.00,?;
129 Stamps 13.6
129 Fruits{} 17.6
129 Market;! 128.00?;
121 Gasoline;! 13.6?;
"#;
let b3 = r#"
1242.00
122 Hardware;! 13.6
127 Hairdresser 13.1
123 Fruits 93.5?;
132 Stamps;!{ 13.6?;
160 Pen;! 17.6?;
002 Car;! 34.00
"#;
let b4 = r#"
1687.0
160 Perfume;! 71.4?;
126 Stamps;! 13.6?;
132 Gasoline;! 54.00?;
003 Hardware;! 93.5?;
130 Gasoline;! 34.00?;
123 Hairdresser;! 12.2?;
"#;
let b5 = r#"
963.0
131 Books 12.2
139 Gasoline 120.90
002 Hardware;! 71.4?;
"#;
let b1sol="Original Balance: 1000.00\n125 Market 125.45 Balance 874.55\n126 Hardware 34.95 Balance 839.60\n127 Video 7.45 Balance 832.15\n128 Book 14.32 Balance 817.83\n129 Gasoline 16.10 Balance 801.73\nTotal expense  198.27\nAverage expense  39.65";
let b2sol="Original Balance: 1233.00\n125 Hardware 24.80 Balance 1208.20\n123 Flowers 93.50 Balance 1114.70\n127 Meat 120.90 Balance 993.80\n120 Picture 34.00 Balance 959.80\n124 Gasoline 11.00 Balance 948.80\n123 Photos 71.40 Balance 877.40\n122 Picture 93.50 Balance 783.90\n132 Tyres 19.00 Balance 764.90\n129 Stamps 13.60 Balance 751.30\n129 Fruits 17.60 Balance 733.70\n129 Market 128.00 Balance 605.70\n121 Gasoline 13.60 Balance 592.10\nTotal expense  640.90\nAverage expense  53.41";
let b3sol="Original Balance: 1242.00\n122 Hardware 13.60 Balance 1228.40\n127 Hairdresser 13.10 Balance 1215.30\n123 Fruits 93.50 Balance 1121.80\n132 Stamps 13.60 Balance 1108.20\n160 Pen 17.60 Balance 1090.60\n002 Car 34.00 Balance 1056.60\nTotal expense  185.40\nAverage expense  30.90";    
let b4sol="Original Balance: 1687.00\n160 Perfume 71.40 Balance 1615.60\n126 Stamps 13.60 Balance 1602.00\n132 Gasoline 54.00 Balance 1548.00\n003 Hardware 93.50 Balance 1454.50\n130 Gasoline 34.00 Balance 1420.50\n123 Hairdresser 12.20 Balance 1408.30\nTotal expense  278.70\nAverage expense  46.45";
let b5sol="Original Balance: 963.00\n131 Books 12.20 Balance 950.80\n139 Gasoline 120.90 Balance 829.90\n002 Hardware 71.40 Balance 758.50\nTotal expense  204.50\nAverage expense  68.17";

            dotest(b1, b1sol);
            dotest(b2, b2sol);
            dotest(b3, b3sol);
            dotest(b4, b4sol);
            dotest(b5, b5sol);
    }
    extern crate rand;
    use self::rand::{Rng};
    
    fn compose(k: i32) -> String {
        let mut rng = rand::thread_rng();
        let ctgr=vec!["Market","Hardware","Video","Books","Music","Gasoline","Beauty","Perfume","Pen","Grocery","Stamps","Photos","Picture","Vegetables","Flowers","Fruits","Hairdresser","Meat","Car", "Tyres"]; // 20
        let prices=vec!["120.3","13.1","17.6","93.5","3.2","71.4","12.2","120.90","34.00","13.6","11.00","12.00","110.7","24.8","54.00","17.5","128.00","17.00","19.00,","20.00"]; // 20
        let chks=vec!["001","002","003","120","121","122","123","124","125","126","127","128","129","130","131","131","132","129","139","160"];
        let bal = (rng.gen_range(800, 2000) as f64) * 1.5;
        let mut res = format!("{:.2}\n", bal);
        let mut i = 0;
        while i < k {
            let cat = ctgr[rng.gen_range(0, ctgr.len())];
            let p = prices[rng.gen_range(0, prices.len())];
            let ck = chks[rng.gen_range(0, chks.len())];
            //res += &format!("{}, {}, {}\n", cat, p, ck);
            res += &format!("{}, {}, {}\n", ck, cat, p);
            i += 1;
        }
        return res[0..res.len()-1].to_string();
    }
    
    fn balance_gz(book: &str) -> String {
        let re1 = oregex::Regex::new(r"[^\n. \dA-Za-z]").unwrap();
        let u = re1.replace_all(book, "");
        let re = oregex::Regex::new(r"\n+").unwrap();
        let mut narr: Vec<&str> = re.split(&u).collect::<Vec<&str>>();
        narr.retain(|x| x.len() != 0);
        let total = match narr[0].parse::<f64>() {
            Ok(total) => total, Err(_e) => { -1.0 }
        };
        let mut current = total;
        let mut res = format!("Original Balance: {:.2}", total);
        let mut i = 1;
        while i < narr.len() {
            let l = narr[i].split(" ").collect::<Vec<&str>>();
            let g = match l[2].parse::<f64>() {
                Ok(g) => g, Err(_e) => { -1.0 }
            };
            current -= g;     
            res += &format!("\n{} {} {:.2} Balance {:.2}", l[0], l[1], g, current);
            i += 1;
        }
        res += &format!("\nTotal expense  {:.2}\nAverage expense  {:.2}", 
                        total - current, (total - current) / ((narr.len() -1) as f64));
        return res;
    }
    
    #[test]
    fn random_tests() {
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let s = &compose(rng.gen_range(5, 10));
            let sol = balance_gz(s);
            dotest(s, &sol);
        }
    }
}
