# Burgers API 🍔

**Burgers API** is a spin-off of https://punkapi.com/documentation/v2, but instead of for beers - for burgers. The API is built with **Rust**, the **Diesel** framework with a postgres database, and of course, the **Rocket** API framework. 

**Burgers API** supports all the basic **CRUD** operations, along with a method that returns a random burger. The API also provides functionality to look up burgers by name, it's important to note that the search is case-insensitive and partial, meaning that if we have *"Pulled pork burger"* and *"Classic pork burger"* in our database, searching with *"PO"* will match both.

The service is hosted on a **DigitalOcean** droplet, running **Ubuntu** 18.04 (LTS) x64, with **NGINX** as a reverse proxy. It also has an **SSL Certificate** is provided by CertBot.

All of these endpoints will be covered below, but there is also a Postman collection that can help you with testing the API out - https://github.com/arthas168/burgers-api/blob/main/Burgers-API.postman_collection.json

## API Reference:

### Root Endpoint:
http://burger-api.ml:8000

The root endpoint itself does not return anything, but it prefixes all other service endpoints.

### List All Burgers:
**GET** `http://burger-api.ml:8000/burgers`

### Get a Burger With a Specific ID:
**GET** `http://burger-api.ml:8000/burgers/<id>`
  
### Add a New Burger:
**POST** `http://burger-api.ml:8000/burgers` with a JSON body that takes a name and a description, for instance:
> {
    "name": "Classic Beef Burger",
    "description": "White bread, beef, veggies, spices"
}

### Update an Existing Burger:
**PUT** `http://burger-api.ml:8000/burgers/<id>` with a JSON body that takes a name and a description, for instance:
> {
    "name": "Pulled pork burger",
    "description": "White bread, pulled pork, veggies, spices"
}
  
### Delete a Burger:
**DELETE** `http://burger-api.ml:8000/burgers/<id>`
  
### Get a Random Burger:
**GET** `http://burger-api.ml:8000/burgers/random`

### Find Bugers by Name:
**GET** `http://burger-api.ml:8000/burgers/name/<name>`
  
*All invalid requests to the API will return the appropriate error.*

## Running Burgers API locally
In order to run this repo locally, you'll need the latest version of Rust nightly, as well as an active postgres DB running locally. Use the .env.example to set up your environment variables and after that simply execute cargo run in the project root directory.

## Testing
All of the endpoints are tested using Rocket's testing utilities. When you've configured the service locally you could to cargo test and hopefully all 8 tests will pass.