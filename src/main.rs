use wowint;

//Codemon

// cargo new hello_wow --bin
// cargo install wowint iid
// cargo add wowint iid
fn main() {

    //CODEMON KEY
    let mut key_arrow_up_down: i32 =1038;
    let mut key_arrow_left_down: i32 =1037;
    let mut key_arrow_right_down: i32 =1039;
    let mut key_arrow_down_down: i32 =1040;

    let mut key_alpha_0_down: i32 =1048;
    let mut key_alpha_1_down: i32 =1049;
    let mut key_alpha_2_down: i32 =1050;
    let mut key_alpha_3_down: i32 =1051;
    let mut key_alpha_4_down: i32 =1052;
    let mut key_alpha_5_down: i32 =1053;

    let mut key_numpad_0_down: i32 =1096;
    let mut key_numpad_1_down: i32 =1097;
    let mut key_numpad_2_down: i32 =1098;
    let mut key_numpad_3_down: i32 =1099;
    let mut key_numpad_4_down: i32 =1100;
    let mut key_numpad_5_down: i32 =1101;



    println!("Hello, UDP IID PUSHER!");
    let mut bool_loop_sender= true;
    if bool_loop_sender{
        let mut server_name= "192.168.43.170";
        let mut use_server = false;
        if use_server {
            server_name = "apint.ddns.net";
        }
        let mut sender = SendUdpIID::new(server_name, 7073, true).unwrap();

        let mut value: i32 = 0;
        let mut index: i32 = 0;
        loop {
            sender.push_index_integer_date_ntp_in_milliseconds(1, value, 0).unwrap();
            if index%10 == 0 {
                std::thread::sleep(std::time::Duration::from_secs(1));
                sender.push_index_integer_date_ntp_in_milliseconds(1,1038,0);
                std::thread::sleep(std::time::Duration::from_secs(1));
                sender.push_index_integer_date_ntp_in_milliseconds(1,2038,0);
            }
            index += 1;
            std::thread::sleep(std::time::Duration::from_secs(1));

        }   
    }
}