use serde::{Deserialize, Serialize};
use validator::Validate;

use orka_proto::scheduler_controller::{
    workload_status::{Resources, Status},
    WorkloadStatus,
};

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct InstanceStatus {
    #[validate(length(min = 1))]
    pub name: String,
    pub status_code: InstanceStatusCode,
    pub resource_usage: InstanceResources,
}

impl From<&WorkloadStatus> for InstanceStatus {
    fn from(status: &WorkloadStatus) -> Self {
        InstanceStatus {
            name: (*status.instance_id).to_string(),
            status_code: InstanceStatusCode::from(status.status.clone()),
            resource_usage: InstanceResources::from(status.resource_usage.clone()),
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct InstanceResources {
    pub cpu: i32,

    pub memory: i32,

    pub disk: i32,
}

impl From<Option<Resources>> for InstanceResources {
    fn from(resources: Option<Resources>) -> Self {
        match resources {
            Some(res) => InstanceResources {
                cpu: res.cpu,
                memory: res.memory,
                disk: res.disk,
            },
            None => InstanceResources {
                cpu: 0,
                memory: 0,
                disk: 0,
            },
        }
    }
}

#[derive(Debug, Validate, Deserialize, Serialize, Clone)]
pub struct InstanceStatusCode {
    pub code: Code,
    pub message: Option<String>,
}

impl From<Option<Status>> for InstanceStatusCode {
    fn from(status: Option<Status>) -> Self {
        match status {
            Some(st) => InstanceStatusCode {
                code: Code::from_i32(st.code),
                message: st.message,
            },
            None => InstanceStatusCode {
                code: Code::WAITING,
                message: Some(String::from("No status found")),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Code {
    WAITING = 0,
    RUNNING = 1,
    TERMINATED = 2,
}

impl Code {
    fn from_i32(value: i32) -> Code {
        match value {
            0 => Code::WAITING,
            1 => Code::RUNNING,
            2 => Code::TERMINATED,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

