// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use actix_web::{
    App,
    HttpServer,
};
// use tokio::task::JoinHandle;
use futures::StreamExt;
use std::time::Duration;
use tokio;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(3)).await;
            tx.send(()).unwrap();
        });

        let server = HttpServer::new(|| {
            App::new()
                    .service(
                        actix_files::Files::new("/", "/tmp")
                                .show_files_listing()
                                .use_hidden_files(),
                    )
        })
                .shutdown_timeout(1)
                .bind(("0.0.0.0", 4804))
                .unwrap()
                .run();


        tokio::select! {
            _ = async {
                server.await.unwrap();
                // Help the rust type inferencer out
                Ok::<_,std::io::Error>(())
            } => {}
            _ = rx => {
                println!("terminating async task");
            }
        }
    }

    tauri_async_rt_app_lib::run().await;
}
