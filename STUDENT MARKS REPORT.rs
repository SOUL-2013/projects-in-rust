fn main(){


    let name:&str ="soumadip biswas";
    let math = 63;

    let bengali = 50;
    let english = 96;
    let history = 59;
    let computer = 100;
    let total = math+bengali+english+history+computer;
    let average = total / 5;
    let grade = 'A';


    println!("===== STUDENT MARKS REPORT =====");

    println!("\nName: {}", name);

    println!("\nMathematics: {}", math);
    println!("bengali: {}",bengali );
    println!("english: {}",english );
    println!("History: {}", history);
    println!("computer: {}", computer);

    println!("\nTotal Marks: {}", total);
    println!("Average Marks: {}", average);
    println!("grade:{}",grade);
    println!("REPORT END");
    println!("================================");

}

