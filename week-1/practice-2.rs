fn grade(score: i32) -> String {
  
    if score >= 90{
        "excellent".to_string()

    }else if score >=70{
        "good job".to_string()

    }else{
        if score < 40{
            "needs remedial practice".to_string()
        }else{ "needs improvement".to_string() }
        
    }

}

fn main(){
  let score = 25;
  let my_grade = grade(score);
  println!("{}", my_grade);

}