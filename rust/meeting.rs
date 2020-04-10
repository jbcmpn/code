//https://www.codewars.com/kata/59df2f8f08c6cec835000012/

/*John has invited some friends. His list is:

s = "Fred:Corwill;Wilfred:Corwill;Barney:Tornbull;Betty:Tornbull;Bjon:Tornbull;Raphael:Corwill;Alfred:Corwill";
Could you make a program that

makes this string uppercase
gives it sorted in alphabetical order by last name.
When the last names are the same, sort them by first name. Last name and first name of a guest come in the result between parentheses separated by a comma.

So the result of function meeting(s) will be:

"(CORWILL, ALFRED)(CORWILL, FRED)(CORWILL, RAPHAEL)(CORWILL, WILFRED)(TORNBULL, BARNEY)(TORNBULL, BETTY)(TORNBULL, BJON)"
It can happen that in two distinct families with the same family name two people have the same first name too.
*/

fn meeting(s: &str) -> String {
    let mut names = s.to_uppercase()
        .split(';')
        .map(|p| {
            let split = &p
                .split(':')
                .collect::<Vec<&str>>();
            format!("({}, {})", split[1], split[0])
        })
        .collect::<Vec<String>>()
    ;
    names.sort();
    names.join("")
}
extern crate rand;
use self::rand::Rng;
//
// basic tests
//
fn dotest(s: &str, exp: &str) -> () {
        println!("s:{}", s);
        let ans = meeting(s);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
}
#[test]
fn basic_tests() {
    dotest("Alexis:Wahl;John:Bell;Victoria:Schwarz;Abba:Dorny;Grace:Meta;Ann:Arno;Madison:STAN;Alex:Cornwell;Lewis:Kern;Megan:Stan;Alex:Korn",
           "(ARNO, ANN)(BELL, JOHN)(CORNWELL, ALEX)(DORNY, ABBA)(KERN, LEWIS)(KORN, ALEX)(META, GRACE)(SCHWARZ, VICTORIA)(STAN, MADISON)(STAN, MEGAN)(WAHL, ALEXIS)");
    dotest("John:Gates;Michael:Wahl;Megan:Bell;Paul:Dorries;James:Dorny;Lewis:Steve;Alex:Meta;Elizabeth:Russel;Anna:Korn;Ann:Kern;Amber:Cornwell",
           "(BELL, MEGAN)(CORNWELL, AMBER)(DORNY, JAMES)(DORRIES, PAUL)(GATES, JOHN)(KERN, ANN)(KORN, ANNA)(META, ALEX)(RUSSEL, ELIZABETH)(STEVE, LEWIS)(WAHL, MICHAEL)");   
    dotest("Alex:Arno;Alissa:Cornwell;Sarah:Bell;Andrew:Dorries;Ann:Kern;Haley:Arno;Paul:Dorny;Madison:Kern",
        "(ARNO, ALEX)(ARNO, HALEY)(BELL, SARAH)(CORNWELL, ALISSA)(DORNY, PAUL)(DORRIES, ANDREW)(KERN, ANN)(KERN, MADISON)");    
    dotest("Anna:Wahl;Grace:Gates;James:Russell;Elizabeth:Rudd;Victoria:STAN;Jacob:Wahl;Alex:Wahl;Antony:Gates;Alissa:Meta;Megan:Bell;Amandy:Stan;Anna:Steve",
        "(BELL, MEGAN)(GATES, ANTONY)(GATES, GRACE)(META, ALISSA)(RUDD, ELIZABETH)(RUSSELL, JAMES)(STAN, AMANDY)(STAN, VICTORIA)(STEVE, ANNA)(WAHL, ALEX)(WAHL, ANNA)(WAHL, JACOB)");    
    dotest("Ann:Russel;John:Gates;Paul:Wahl;Alex:Tolkien;Ann:Bell;Lewis:Kern;Sarah:Rudd;Sydney:Korn;Madison:Meta",
        "(BELL, ANN)(GATES, JOHN)(KERN, LEWIS)(KORN, SYDNEY)(META, MADISON)(RUDD, SARAH)(RUSSEL, ANN)(TOLKIEN, ALEX)(WAHL, PAUL)");    
    dotest("Paul:Arno;Matthew:Schwarz;Amandy:Meta;Grace:Meta;Ann:Arno;Alex:Schwarz;Jacob:Rudd;Amber:Cornwell",
        "(ARNO, ANN)(ARNO, PAUL)(CORNWELL, AMBER)(META, AMANDY)(META, GRACE)(RUDD, JACOB)(SCHWARZ, ALEX)(SCHWARZ, MATTHEW)");    
    dotest("Abba:Bell;Lewis:Cornwell;Jacob:STAN;Matthew:Schwarz;Ann:STAN;Sophia:Gates;Victoria:Korn;Anna:Bell;Paul:Kern;Alissa:Tolkien",
        "(BELL, ABBA)(BELL, ANNA)(CORNWELL, LEWIS)(GATES, SOPHIA)(KERN, PAUL)(KORN, VICTORIA)(SCHWARZ, MATTHEW)(STAN, ANN)(STAN, JACOB)(TOLKIEN, ALISSA)");   
    dotest("Victoria:Thorensen;Ann:Arno;Alexis:Wahl;Emily:Stan;Anna:STAN;Jacob:Korn;Sophia:Gates;Amber:Kern",
        "(ARNO, ANN)(GATES, SOPHIA)(KERN, AMBER)(KORN, JACOB)(STAN, ANNA)(STAN, EMILY)(THORENSEN, VICTORIA)(WAHL, ALEXIS)");   
    dotest("Andrew:Arno;Jacob:Russell;Anna:STAN;Antony:Gates;Amber:Korn;Lewis:Dorries;Ann:Burroughs;Alex:Kern;Anna:Arno;Elizabeth:Kern;John:Schwarz;Sarah:STAN",
        "(ARNO, ANDREW)(ARNO, ANNA)(BURROUGHS, ANN)(DORRIES, LEWIS)(GATES, ANTONY)(KERN, ALEX)(KERN, ELIZABETH)(KORN, AMBER)(RUSSELL, JACOB)(SCHWARZ, JOHN)(STAN, ANNA)(STAN, SARAH)");  
    dotest("Megan:Wahl;Alexis:Arno;Alex:Wahl;Grace:STAN;Amber:Kern;Amandy:Schwarz;Alissa:Stan;Paul:Kern;Ann:Meta;Lewis:Burroughs;Andrew:Bell",
        "(ARNO, ALEXIS)(BELL, ANDREW)(BURROUGHS, LEWIS)(KERN, AMBER)(KERN, PAUL)(META, ANN)(SCHWARZ, AMANDY)(STAN, ALISSA)(STAN, GRACE)(WAHL, ALEX)(WAHL, MEGAN)");
}
//
// random tests
//
static FNAMES: &'static [&'static str] = &[
    "Emily", "Sophia", "Anna", "Anna", "Sarah", "Michael", "Jacob", "Alex", "Alex", "Alex", "Antony", "John", "Matthew", "Andrew", "Paul", "Paul", "Ann", "Ann", "Ann", "Ann", "Robert", 
    "Megan", "Alissa", "Alexis", "Grace", "Madison", "Elizabeth", "James", "Amandy", "Abba", "Victoria", "Amber", "Sydney", "Haley", "Lewis"
];
static LNAMES: &'static [&'static str] = &[
    "Korn", "Arno", "Arno", "Bell", "Bell", "Kern", "Kern", "Kern", "Russel", "Meta", "Meta", "Meta", "Cornwell", "Cornwell", "Wahl", "Wahl", "Wahl", "Wahl", "Dorny", "Dorries", 
       "Stan", "STAN", "STAN", "Thorensen", "Schwarz", "Schwarz", "Gates", "Steve", "Tolkien", "Burroughs", "Gates", "Bell", "Korn", "Russell", "Rudd"
];
fn meeting39(s: &str) -> String {
    let mut names = s
        .split(';')
        .map(|person| {
            let split = &person
                .to_uppercase()
                .split(':')
                .map(|i| i.to_string())
                .collect::<Vec<String>>();
            format!("({}, {})", split[1], split[0])
        })
        .collect::<Vec<String>>()
    ;
    names.sort();
    names.join("")
}
fn compose39(k: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut items = vec![];
    for _ in 0..k {
        let fname = FNAMES[rng.gen_range(0, FNAMES.len())];
        let lname = LNAMES[rng.gen_range(0, FNAMES.len())];
        items.push(format!("{}:{}", fname, lname));
    }
    items.join(";")
}
#[test]
fn random_tests() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let amount = rng.gen_range(6, 13);
        let given = compose39(amount);
        let expected = meeting39(&given);
        assert_eq!(meeting(&given), expected);
    }
}

