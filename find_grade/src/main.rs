use std::io;

fn main() {
    let mut lab_score = String::new();
    let mut exam_score = String::new();
    
    // รับคะแนนแลปและคะแนนสอบ
    println!("Enter your lab score (40 Point): ");
    io::stdin().read_line(&mut lab_score).expect("Failed to read line");
    let lab_score: f64 = lab_score.trim().parse().expect("Please enter a valid number");
    
    println!("Enter your exam score (60 Point): ");
    io::stdin().read_line(&mut exam_score).expect("Failed to read line");
    let exam_score: f64 = exam_score.trim().parse().expect("Please enter a valid number");
    
    // คำนวณคะแนนรวม
    let total_score = lab_score + exam_score;
    
    // คำนวณเกรดโดยใช้ match expression
    let grade = match total_score {
        _ if total_score >= 90.0 => 'A',
        _ if total_score >= 80.0 => 'B',
        _ if total_score >= 70.0 => 'C',
        _ if total_score >= 60.0 => 'D',
        _ => 'F',
    };
    
    // แสดงผลลัพธ์
    println!("Your grade is: {}", grade);
}
