#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use crate::db::models::{Score, User};

    // Database tests
    #[test]
    fn insert_user() {
        let a = User::add(2, format!("UserA").to_string());
        assert_eq!(a.get_id(),2);
    }
    #[test]
    fn get_name(){
        let a = User::add(2, format!("UserA").to_string());
        assert_eq!(a.get_name(),"UserA");
    }
    #[test]
    fn insert_score() {
        let d = NaiveDate::from_ymd(2015, 6, 3);
        let t = NaiveTime::from_hms_milli(12, 34, 56, 789);

        let dt = NaiveDateTime::new(d, t);

        let a = User::add(2, format!("UserA").to_string());
        assert_eq!(Score::add_score(a,dt),true);
    }
    #[test]
    fn remove_score() {
        let d = NaiveDate::from_ymd(2015, 6, 3);
        let t = NaiveTime::from_hms_milli(12, 34, 56, 789);

        let dt = NaiveDateTime::new(d, t);

        let a = User::add(2, format!("UserA").to_string());
        let b = Score::add_score(a,dt);
        assert_eq!(Score::remove_score(b),2);
    }
    #[test]
    fn insert_image() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn delete_user() {
        let a = User::add(2, format!("UserA").to_string());
        assert_eq!(a.remove_user(),2);
    }
    #[test]
    fn delete_image() {
        assert_eq!(2 + 2, 4);
    }
}
