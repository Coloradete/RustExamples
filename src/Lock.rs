use std::io::stdin;

enum State{
    Locked,
    Failed,
    Unlocked
}

pub(crate) fn try_lock()
{
    let code = String::from("1234");

    let mut state = State::Locked;

    let mut entry = String::new();

    loop{
        match state{
            State::Locked =>{
                let mut input = String::new();

                // let s = & input;
                // let s1 = &mut input;
                //
                // s1.push_str("aa");
                //
                // println!("{}",s);

                match stdin().read_line(&mut input){
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_)=>{

                    }
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                    continue;
                }
            }

            State::Failed => {
                println!("Try again madafaka");
                entry.clear();
                state = State::Locked;
                continue;
            }

            State::Unlocked =>{
                println!("BIEEEN");
                entry.clear();
                state = State::Locked;
                return;
            }
        }
    }
}
