use axum::{response::Html, Json};
use serde_json::{self, Value, json};
use crate::{client::{Client, scheduler::{SchedulingRequest, Workload, workload::{Type, Resources}}}, errors::ApiError};
use crate::types::Workload_Request::Workload_Request;
use validator::Validate;
use anyhow::{Context};
use log::info;

pub async fn get_workloads(body: String) -> Html<String> {
    tokio::spawn(async move {
        // TODO: Implement => retrieve data from dbValueValue
    });
    Html(format!("Hello, {}!", {"body"}))
}


pub async fn post_workload(body: String) -> anyhow::Result<Json<Value>, ApiError> {
    // We spawn a thread to handle the request
        let mut client = Client::new().await?;
        // Create a new Workload Request object out of the body
        let json_body: Workload_Request = serde_json::from_str(&body)?;

        // Validate the request
        json_body.validate()?;
        
        // Extract the env variable table
        let mut environment = Vec::new();
        if json_body.workload.environment.len() > 0 {
            for env in json_body.workload.environment.iter() {
                environment.push(env.clone());
            }
        }
        
        // Create a grpc workload object
        let workload = Workload {
            name: json_body.workload.name,
            r#type: Type::Container.into(),
            image: json_body.workload.image,
            environment: environment,
            resource_limits: Some(Resources{
                cpu: Some(1 as i32),
                memory: Some(1 as i32),
                disk: Some(1 as i32),    
            }),
        };

        let request = SchedulingRequest {
            workload: Some(workload),
        };

        client.schedule_workload(request).await.unwrap();
        // TODO: Handle the grpc response and if OK save data and send response to cli            
        return Ok(Json(json!({"description": "Created"})));
    
}