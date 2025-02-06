use mysql::*;

/// The function error_tester() test the server dependencies.
pub fn error_tester() {
    println!("🚀 Error tester running...");
    println!("🛠️ Procedure for testing server dependencies...");

    
    bd_test();

    println!("Error check completed!🎉");
 
}

/// The function bd_test() test the connection to database server.
fn bd_test() {

    let url = "mysql://wore_user:Q9NXsREfSEcrwJV!@localhost:3306/wrereg";
    let pool = Pool::new(url).expect("Failed to connect to database");
    let _conn = pool.get_conn().expect("Failed to get connection");
    println!("✅ Database connection test passed!");

}
