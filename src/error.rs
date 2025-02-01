use mysql::*;

pub fn error_tester() {
    println!("🚀 Error tester running...");
    println!("🛠️ Procedure for testing server dependencies...");
    

    bd_test();

}

fn bd_test() {


    let url = "mysql://user:password@localhost:3306/bd_name";
    let pool = Pool::new(url).expect("Failed to create pool");
    let _conn = pool.get_conn().expect("Failed to establish connection");
    println!("Ok ✅ ");

}