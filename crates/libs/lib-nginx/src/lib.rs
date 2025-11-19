#![allow(unused)]

use serde::Serialize;

/// https://nginx.org/en/docs/ngx_core_module.html#pcre_jit
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "lowercase")]
enum OnOff {
    #[default]
    Off,
    On,
}

/// https://nginx.org/en/docs/ngx_core_module.html#worker_processes
#[derive(Debug, Clone, Default, Serialize)]
enum WorkerProcesses {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    U8(u8),
}

struct Events {
    /// https://nginx.org/en/docs/ngx_core_module.html#worker_connections
    worker_connections: u32,
}

struct NginxConfig {
    worker_processes: WorkerProcesses,
    pcre_jit: OnOff,
    events: Events,
}
