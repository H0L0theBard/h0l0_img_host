# h0l0_img_host
A backend image hosting service made in Rust.


# Setup

Using `cargo run --release` will build and run a release version of the application and upon first run, an api key will print into the terminal. 
This initial key provided will be for the admin account 

# Endpoints 
```
GET /stats - Statistics endpoint used to show data about the host
POST /api/usr/new - New account creation (can only be done by an admin)
POST /api/img - Gets all images for an account from the key provided
POST /api/upload - Use a multipart/form to upload an image
DELETE /api/delete - Delete a file 
DELETE /api/nuke - Nukes all images off your account
```
