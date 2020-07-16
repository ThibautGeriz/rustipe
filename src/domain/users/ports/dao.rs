trait UserDao {
    fn signup(email: String, password: String);
    fn signin(email: String, password: String);
}
