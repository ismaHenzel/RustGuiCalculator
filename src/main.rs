#[macro_use]
extern crate web_view;
extern crate meval;
use web_view::*;
use meval::*;

#[derive(Debug)]
pub struct Calc{
    arr_expression: Vec<String>,
    last_operation: String,
}
impl Calc{
    fn clear_data(&mut self){
        self.last_operation = String::from("");
        self.arr_expression = vec!["".to_string()];
    }

    fn get_last_value(&self)->String{
        self.arr_expression[self.arr_expression.len()-1].to_string()
    }

    fn set_last_value(&mut self, value: String)
    {
        let len = self.arr_expression.len()-1;
        self.arr_expression[len] = value
    }

    fn is_operator(value: &str)-> bool{
        let operators = ["+","-","*","/","%"];
        operators.contains(&value)
    }

    fn add_operator(&mut self, value: String){
        let last_value = self.get_last_value();
        if !last_value.parse::<f64>().is_ok(){
            if Self::is_operator(&value) && Self::is_operator(&last_value){
                self.set_last_value(value)
            }else{
                self.arr_expression.push(value);
            }
        }else{
            if Self::is_operator(&value){
                self.arr_expression.push(value);
            }else{
                let current_val = self.get_last_value()+&value;
                self.set_last_value(current_val);
            }
        }
    }
    fn resolve(&mut self, value: String){    
        let op = &self.arr_expression.join("");    
        let expression = meval::eval_str(op);
        let result: String = match expression
        {
            Ok(expression)=> expression.to_string(),
            Err(_)=> "ERRO".to_string()
        };
        Self::clear_data(self);
        self.last_operation = format!("{:?}={:?}",op,result);
        if(result=="ERRO"){
            self.set_last_value("".to_string());
        }else{
            self.set_last_value(result);
        }
    }

    fn remove(&mut self){
        let mut data= self.get_last_value();
        if self.arr_expression.len() <= 1{
            if(data.len()!=0)
            {
                data.pop();
                self.set_last_value(data);
            }else{
                self.set_last_value("".to_string());
            }
        }else if data.len() <=1 {
            self.arr_expression.pop();
        }else{
                data.pop();
                self.set_last_value(data); 
        }
    }
}
fn main() {
    let html = format!(
        include_str!("index.html"),
        styles = include_str!("styles/style.css"),
        scripts = include_str!("scripts/manipulationDOM.js"),
    );

    web_view::builder()
        .title("Calculator")
        .content(Content::Html(html))
        .size(500, 600)
        .resizable(true)
        .debug(true)
        .user_data(Calc{
            arr_expression: vec!["".to_string()],
            last_operation: "".to_string()
        })
        .invoke_handler(|webview,arg|{
            let mut calc = webview.user_data_mut();
            match arg{
                "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"." => calc.add_operator(arg.to_string()),
                "+"| "-"| "*"| "/"| "%"=> calc.add_operator(arg.to_string()),
                "("|")" => calc.add_operator(arg.to_string()),
                "="|"Enter" => calc.resolve(arg.to_string()),
                "Delete" => calc.clear_data(),
                "Backspace"=> calc.remove(),
                err=> (()),
            };

            let mut value = calc.arr_expression.join("");
            if value.len() <= 0 {
                value = "0".to_string();
            }

            let display_value = format!("getData({:?})", value);
            let last_op_value = format!("last_operation({:?})", calc.last_operation);
            webview.eval(&display_value);
            webview.eval(&last_op_value);
            Ok(())
        })
        .run()
        .unwrap();
}

