use crate::{
    api_service::fetch_vector_neighbors,
    models::{
        rpc::{FetchNeighbors, RPCResponseBody, Vector, VectorIdValue},
        types::VectorId,
    },
    web_server::AppEnv,
};
use actix_web::{web, HttpResponse};

// Route: `/vectordb/fetch`
pub(crate) async fn fetch(
    env: web::Data<AppEnv>,
    web::Json(body): web::Json<FetchNeighbors>,
) -> HttpResponse {
    // Try to get the vector store from the environment
    let vec_store = match env.vector_store_map.get(&body.vector_db_name) {
        Some(store) => store,
        None => {
            // Vector store not found, return an error response
            return HttpResponse::InternalServerError().body("Vector store not found");
        }
    };
    let fvid = VectorId::from(body.vector_id);

    let result = fetch_vector_neighbors(vec_store.clone(), fvid).await;

    let mut xx: Vec<Option<RPCResponseBody>> = result
        .iter()
        .map(|res_item| {
            res_item.as_ref().map(|(vect, neig)| {
                let nvid = VectorIdValue::from(vect.clone());
                let response_data = RPCResponseBody::RespFetchNeighbors {
                    neighbors: neig
                        .iter()
                        .map(|(vid, x)| (VectorIdValue::from(vid.clone()), *x))
                        .collect(),
                    vector: Vector {
                        id: nvid,
                        values: vec![],
                    },
                };
                response_data
            })
        })
        .collect();
    // Filter out any None values (optional)
    xx.retain(|x| x.is_some());
    let rs: Vec<RPCResponseBody> = xx.into_iter().map(|x| x.unwrap()).collect();
    HttpResponse::Ok().json(rs)
}
