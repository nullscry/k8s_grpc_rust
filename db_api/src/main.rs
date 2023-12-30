use tonic::{transport::Server, Request, Response, Status};

pub mod dbapi {
    tonic::include_proto!("dbapi");
}
pub mod models;
mod query;
pub mod schema;

use dbapi::{
    database_server::{Database, DatabaseServer},
    select_feature_reply::FeatureRow,
    select_model_reply::ModelRow,
    InsertFeatureRequest, InsertModelRequest, InsertReply, SelectFeatureReply,
    SelectFeatureRequest, SelectModelReply, SelectModelRequest,
};

use query::{create_feature, create_model, select_features, select_models};

// #[derive(Clone)]
// struct DatabaseService {
//     connection_pool: Pool<ConnectionManager<PgConnection>>,
// }

// impl DatabaseService {
//     fn new() -> Result<Self, diesel::result::Error> {
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let manager = ConnectionManager::<PgConnection>::new(database_url);
//         let pool = Pool::builder().build(manager)?;

//         Ok(DatabaseService { connection_pool: pool })
//     }
// }

#[derive(Debug, Default)]
pub struct DatabaseService {}

#[tonic::async_trait]
impl Database for DatabaseService {
    async fn insert_model(
        &self,
        request: Request<InsertModelRequest>,
    ) -> Result<Response<InsertReply>, Status> {
        let model_request = request.into_inner();

        println!("Received {:?}", model_request);

        let inserted_model = create_model(model_request);

        Ok(Response::new(InsertReply {
            status: format!(
                "Model inserted successfully: id: {}, name: {}, category: {}",
                inserted_model.id, inserted_model.name, inserted_model.category
            )
            .into(),
        }))
    }

    async fn insert_feature(
        &self,
        request: Request<InsertFeatureRequest>,
    ) -> Result<Response<InsertReply>, Status> {
        let feature_request = request.into_inner();

        println!("Received {:?}", feature_request);

        let inserted_feature = create_feature(feature_request);

        Ok(Response::new(InsertReply {
            status: format!(
                "Feature inserted successfully: id: {}, name: {}, category: {}, model: {}",
                inserted_feature.id,
                inserted_feature.name,
                inserted_feature.category,
                inserted_feature.model
            )
            .into(),
        }))
    }

    async fn select_model(
        &self,
        request: Request<SelectModelRequest>,
    ) -> Result<Response<SelectModelReply>, Status> {
        let model_request = request.into_inner();

        println!("Received {:?}", model_request);

        let models = select_models(model_request);

        let model_rows: Vec<ModelRow> = models
            .iter()
            .map(|model| ModelRow {
                id: model.id,
                name: model.name.to_string(),
                category: model.category.to_string(),
            })
            .collect();

        Ok(Response::new(SelectModelReply { models: model_rows }))
    }

    async fn select_feature(
        &self,
        request: Request<SelectFeatureRequest>,
    ) -> Result<Response<SelectFeatureReply>, Status> {
        let feature_request = request.into_inner();

        println!("Received {:?}", feature_request);

        let features = select_features(feature_request);

        let features_rows: Vec<FeatureRow> = features
            .iter()
            .map(|feature| FeatureRow {
                id: feature.id,
                name: feature.name.to_string(),
                category: feature.category.to_string(),
                model: feature.model.to_string(),
            })
            .collect();

        Ok(Response::new(SelectFeatureReply {
            features: features_rows,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8888".parse()?;
    let database_service = DatabaseService::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(DatabaseServer::new(database_service))
        .serve(addr)
        .await?;

    Ok(())
}
