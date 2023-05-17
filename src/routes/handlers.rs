use crate::DbPool;
use actix_web::{ get, Error, HttpResponse, post, put, delete, web };
use crate::models::business_model::{BusinessPayload, NewBusiness, Business};


#[get("/health_check")]
async fn health_check() ->Result<HttpResponse,Error>{
    // API health_check
    Ok(HttpResponse::Ok().json("ACTIX-WEB API IS UP AND RUNNING"))
}

/*

    ------------- BUSINESS HANDLER FUNCTIONS  ----------

*/

#[get("/")]
async fn index() ->Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().json("Hello World Rustaceean"))
}


#[post("/businesses/")]
async fn add_business(payload: web::Json<BusinessPayload>,pool: web::Data<DbPool>) ->Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().json("Hello World add_business",))
}


#[get("/businesses")]
async fn get_businesses() ->Result<HttpResponse,Error>{
    // TODO: endpoint to fetch all the businesses registered in Kikao
    Ok(HttpResponse::Ok().json("Fetch all businesses"))
}

#[get("/businesses/{id}")]
async fn get_single_business() ->Result<HttpResponse,Error>{
    // TODO: endpoint to fetch asingle registered business
    Ok(HttpResponse::Ok().json("Fetch a single businesses"))
}

#[put("/businesses/{id}")]
async fn update_business() ->Result<HttpResponse,Error>{
    // TODO: endpoint to update a business
    Ok(HttpResponse::Ok().json("Hello World update_business",))
}

#[delete("/businesses/{id}")]
async fn delete_business() ->Result<HttpResponse,Error>{
    // TODO: endpoint to delete business
    Ok(HttpResponse::Ok().json("Hello World delete_business",))
}


/*

    ------------- REVIEWS HANDLER FUNCTIONS  ----------

*/



#[get("/reviews")]
async fn get_reviews() ->Result<HttpResponse,Error>{
    // TODO: endpoint to fetch all the reviews registered in Kikao
    Ok(HttpResponse::Ok().json("Fetch all reviews"))
}

#[post("/reviews/{id}")]
async fn add_review() ->Result<HttpResponse,Error>{
    // TODO: endpoint to add a review to kikao
    Ok(HttpResponse::Ok().json("Hello World add_review",))
}

#[get("/reviews/{id}")]
async fn get_single_review() ->Result<HttpResponse,Error>{
    // TODO: endpoint to fetch asingle registered review
    Ok(HttpResponse::Ok().json("Fetch a single review"))
}

#[put("/reviews/{id}")]
async fn update_review() ->Result<HttpResponse,Error>{
    // TODO: endpoint to update a review
    Ok(HttpResponse::Ok().json("Hello World update_review",))
}

#[delete("/reviews/{id}")]
async fn delete_review() ->Result<HttpResponse,Error>{
    // TODO: endpoint to delete review
    Ok(HttpResponse::Ok().json("Hello World delete_review",))
}



/*

    ------------- USER HANDLER FUNCTIONS  ----------

*/



#[get("/user/{id}")]
async fn get_user() ->Result<HttpResponse,Error>{
    // TODO: endpoint to fetch asingle user details
    Ok(HttpResponse::Ok().json("Fetch a single user details"))
}

#[post("/user/{id}")]
async fn add_user() ->Result<HttpResponse,Error>{
    // TODO: endpoint to add a user to kikao
    Ok(HttpResponse::Ok().json("Hello World add_user",))
}


