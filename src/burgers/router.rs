use crate::burgers;
use crate::connection;
use rocket;

fn setup_rocket() -> rocket::Rocket {
    rocket::ignite().manage(connection::init_pool()).mount(
        "/burgers",
        routes![
            burgers::handler::all,
            burgers::handler::get,
            burgers::handler::find_by_name,
            burgers::handler::rand,
            burgers::handler::post,
            burgers::handler::put,
            burgers::handler::delete
        ],
    )
}

pub fn create_routes() {
    setup_rocket().launch();
}

#[cfg(test)]
mod test {
    use super::setup_rocket;
    use crate::burgers::Burger;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    #[test]
    fn api_root() {
        let client =
            Client::new(setup_rocket()).expect("Root returns 404 since there is nothing there");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn add_burger() {
        let client = Client::new(setup_rocket()).expect("valid rocket instance");

        let burger_json = r#"
        {
            "name": "Jane Doe Burger",
            "description": "lorem ipsum"
        }"#;

        let mut response = client
            .post("/burgers")
            .body(burger_json)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Created);

        let burger: Burger =
            serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
        assert_eq!(burger.name, "Jane Doe Burger");
        assert_eq!(burger.description, "lorem ipsum");

        client
            .delete(format!("{}{}", "/burgers/", burger.id))
            .dispatch();
    }

    #[test]
    fn list_burgers() {
        let client = Client::new(setup_rocket()).expect("valid rocket instance");

        let burger_json = r#"
        {
            "name": "Jane Doe Burger",
            "description": "lorem ipsum"
        }"#;

        let mut response = client
            .post("/burgers")
            .body(burger_json)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Created);

        let burger: Burger =
            serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

        let mut response = client.get("/burgers").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body_string().unwrap().contains("Jane Doe Burger"));

        client
            .delete(format!("{}{}", "/burgers/", burger.id))
            .dispatch();
    }

    #[test]
    fn update_burger() {
        let client = Client::new(setup_rocket()).expect("valid rocket instance");

        let initial_burger_json = r#"
        {
            "name": "Bad Burger",
            "description": "lorem ipsum"
        }"#;

        let mut response = client
            .post("/burgers")
            .body(initial_burger_json)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Created);

        let initial_burger: Burger =
            serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

        let updated_burger_json = r#"
            {
                "name": "Good Burger",
                "description": "best burger ever"
            }"#;

        let mut response = client
            .put(format!("{}{}", "/burgers/", initial_burger.id))
            .body(updated_burger_json)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let updated_burger: Burger =
            serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
        assert_eq!(updated_burger.name, "Good Burger");
        assert_eq!(updated_burger.description, "best burger ever");

        client
            .delete(format!("{}{}", "/burgers/", updated_burger.id))
            .dispatch();
    }

    #[test]
    fn get_burger_by_id() {
        let client = Client::new(setup_rocket()).expect("valid rocket instance");

        let burger_json = r#"
        {
            "name": "Decent Burger",
            "description": "lorem ipsum"
        }"#;

        let mut response = client
            .post("/burgers")
            .body(burger_json)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Created);

        let burger: Burger =
            serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

        let mut response = client
            .get(format!("{}{}", "/burgers/", burger.id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let burger_response: Burger =
            serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
        assert_eq!(burger_response.name, burger.name);
        assert_eq!(burger_response.description, burger.description);
        assert_eq!(burger_response.id, burger.id);

        client
            .delete(format!("{}{}", "/burgers/", burger_response.id))
            .dispatch();
    }

    #[test]
    fn get_random_burger() {
        let client = Client::new(setup_rocket()).expect("valid rocket instance");
        let burger_json = r#"
        {
            "name": "Ran Dom",
            "description": "lorem ipsum"
        }"#;

        let mut response = client
            .post("/burgers")
            .body(burger_json)
            .header(ContentType::JSON)
            .dispatch();

        let burger: Burger =
            serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

        let client = Client::new(setup_rocket()).expect("valid rocket instance");
        let response = client.get("/burgers/random").dispatch();
        assert_eq!(response.status(), Status::Ok);

        client
            .delete(format!("{}{}", "/burgers/", burger.id))
            .dispatch();
    }

    #[test]
    fn delete_burger() {
        let client = Client::new(setup_rocket()).expect("valid rocket instance");

        let burger_json = r#"
        {
            "name": "Worst burger on earth",
            "description": "Must be deleted"
        }"#;

        let mut response = client
            .post("/burgers")
            .body(burger_json)
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Created);

        let burger: Burger =
            serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();
        assert_eq!(burger.name, "Worst burger on earth");
        assert_eq!(burger.description, "Must be deleted");

        let response = client
            .delete(format!("{}{}", "/burgers/", burger.id))
            .dispatch();
        assert_eq!(response.status(), Status::NoContent);

        let response = client
            .get(format!("{}{}", "/burgers/", burger.id))
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn find_burger_by_name() {
        let client = Client::new(setup_rocket()).expect("valid rocket instance");

        let waldo_burger_json = r#"
        {
            "name": "Waldo burger",
            "description": "red and white stripes"
        }"#;

        let wally_burger_json = r#"
        {
            "name": "Wally burger",
            "description": "red and white stripes"
        }"#;

        let mut waldo_burger_response = client
            .post("/burgers")
            .body(waldo_burger_json)
            .header(ContentType::JSON)
            .dispatch();

        let waldo_burger: Burger =
            serde_json::from_str(waldo_burger_response.body_string().unwrap().as_str()).unwrap();

        let mut wally_burger_response = client
            .post("/burgers")
            .body(wally_burger_json)
            .header(ContentType::JSON)
            .dispatch();

        let wally_burger: Burger =
            serde_json::from_str(wally_burger_response.body_string().unwrap().as_str()).unwrap();

        let mut response = client.get("/burgers/name/wa").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body_string().unwrap().contains("Waldo burger"));

        let mut response = client.get("/burgers/name/wa").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body_string().unwrap().contains("Wally burger"));

        let response = client
            .delete(format!("{}{}", "/burgers/", waldo_burger.id))
            .dispatch();
        assert_eq!(response.status(), Status::NoContent);

        let response = client
            .delete(format!("{}{}", "/burgers/", wally_burger.id))
            .dispatch();
        assert_eq!(response.status(), Status::NoContent);
    }
}
