use std::fs;
// use std::io::BufRead;
use std::process::{Stdio, Command};

use serde::{Deserialize, Serialize};
use serde_json::json;
// use tokio::sync::mpsc;
use tokio::task;
use warp::Filter;
use crate::error_responses::InternalServerError;
use crate::Logger;

// DiscographyQuery
#[derive(Debug, Deserialize, Serialize)]
pub struct DiscographyQuery {
    artists: String,
}

pub fn get_run_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("v1").and(warp::path("run"))
        // update/kcomebacks
        // update/projects
        // update/makediscography?artists=artist1,artist2,artist3
        // update/synclikedsongs
        .and((warp::path("kcomebacks").and_then(update_kcomebacks))
        .or(warp::path("projects").and_then(update_projects))
        .or(warp::path("makediscography").map(||"Not implemented yet"))
        .or(warp::path("synclikedsongs").and_then(sync_liked_songs))
        )
}

async fn update_kcomebacks() -> Result<impl warp::Reply, warp::Rejection> {
    // check if the local repository exists, if not, clone it
    if !fs::metadata("./resources/turbo_octo_potato").is_ok() {
        setup().unwrap();
    };

    if let Err(err) = run_kcomebacks_command() {
        // Handle the error here
        eprintln!("Error: {}", err);
        // Return an appropriate response or error
        return Err(warp::reject::custom(InternalServerError));
    }

    Ok(warp::reply::json(&json!({"status": "updating..."})))

}

async fn update_projects() -> Result<impl warp::Reply, warp::Rejection> {
    // check if the local repository exists, if not, clone it
    if !fs::metadata("./resources/turbo_octo_potato").is_ok() {
        setup().unwrap();
    };

    if let Err(err) = run_projects_command() {
        // Handle the error here
        eprintln!("Error: {}", err);
        // Return an appropriate response or error
        return Err(warp::reject::custom(InternalServerError));
    }

    Ok(warp::reply::json(&json!({"status": "updating..."})))

}

async fn sync_liked_songs() -> Result<impl warp::Reply, warp::Rejection> {
    // check if the local repository exists, if not, clone it
    if !fs::metadata("./resources/turbo_octo_potato").is_ok() {
        setup().unwrap();
    };

    if let Err(err) = run_likedsongs_command() {
        // Handle the error here
        eprintln!("Error: {}", err);
        // Return an appropriate response or error
        return Err(warp::reject::custom(InternalServerError));
    }

    Ok(warp::reply::json(&json!({"status": "updating..."})))

}

pub fn setup() -> Result<(), git2::Error> {
    let repository_url = "https://github.com/JonasunderscoreJones/turbo-octo-potato.git";
    let local_directory = "resources/turbo_octo_potato";

    git2::Repository::clone(repository_url, local_directory)?;

    Ok(())
}

// fn run_command() -> Result<(), std::io::Error> {
//     let (tx, mut rx) = mpsc::channel(1);

//     task::spawn_blocking(move || {
//         let mut child = Command::new("python3")
//             .arg(&py_file)
//             .arg(&args)
//             .current_dir("resources/turbo_octo_potato")
//             .stdout(Stdio::piped())
//             .spawn()
//             .expect("failed to execute child");

//         let stdout = child.stdout.as_mut().unwrap();

//         let mut reader = std::io::BufReader::new(stdout);

//         let mut line = String::new();

//         loop {
//             let len = reader.read_line(&mut line).unwrap();
//             if len == 0 {
//                 break;
//             }
//             tx.blocking_send(line.clone()).unwrap();
//             line.clear();
//         }

//         child.wait().unwrap();
//     });

//     task::spawn(async move {
//         while let Some(line) = rx.recv().await {
//             println!("{}", line);
//         }
//     });

//     Ok(())
// }

// run_command with python file and args as parameters


pub fn run_kcomebacks_command() -> Result<(), std::io::Error> {
    // let (tx, mut rx) = mpsc::channel(1);

    task::spawn_blocking(move || {
        let mut child = Command::new("python3")
            .arg("rpopfetch.py")
            .arg("--cdn")
            .current_dir("resources/turbo_octo_potato")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        // let stdout = child.stdout.as_mut().unwrap();

        // let mut reader = std::io::BufReader::new(stdout);

        // let mut line = String::new();

        // loop {
        //     let len = reader.read_line(&mut line).unwrap();
        //     if len == 0 {
        //         break;
        //     }
        //     tx.blocking_send(line.clone()).unwrap();
        //     line.clear();
        // }

        child.wait().unwrap();
    });

    // task::spawn(async move {
    //     while let Some(line) = rx.recv().await {
    //         Logger::info(&format!("[/v1/kcomebacks/update]: {}", line));
    //     }
    // });
    Logger::info("Updating kcomebacks...");

    Ok(())
}

pub fn run_projects_command() -> Result<(), std::io::Error> {
    // let (tx, mut rx) = mpsc::channel(1);

    task::spawn_blocking(move || {
        let mut child = Command::new("python3")
            .arg("update_projects.py")
            .arg("--cdn")
            .current_dir("resources/turbo_octo_potato")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        // let stdout = child.stdout.as_mut().unwrap();

        // let mut reader = std::io::BufReader::new(stdout);

        // let mut line = String::new();

        // loop {
        //     let len = reader.read_line(&mut line).unwrap();
        //     if len == 0 {
        //         break;
        //     }
        //     tx.blocking_send(line.clone()).unwrap();
        //     line.clear();
        // }

        child.wait().unwrap();
    });

    // task::spawn(async move {
    //     while let Some(line) = rx.recv().await {
    //         Logger::info(&format!("[/v1/projects/update]: {}", line));
    //     }
    // });
    Logger::info("Updating projects...");

    Ok(())
}

pub fn run_likedsongs_command() -> Result<(), std::io::Error> {
    // let (tx, mut rx) = mpsc::channel(1);

    task::spawn_blocking(move || {
        let mut child = Command::new("python3")
            .arg("likedsongsync2.py")
            .current_dir("resources/turbo_octo_potato")
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        // let stdout = child.stdout.as_mut().unwrap();

        // let mut reader = std::io::BufReader::new(stdout);

        // let mut line = String::new();

        // loop {
        //     let len = reader.read_line(&mut line).unwrap();
        //     if len == 0 {
        //         break;
        //     }
        //     tx.blocking_send(line.clone()).unwrap();
        //     line.clear();
        // }

        child.wait().unwrap();
    });

    // task::spawn(async move {
    //     while let Some(line) = rx.recv().await {
    //         Logger::info(&format!("[/v1/synclikedsongs]: {}", line));
    //     }
    // });
    Logger::info("Syncing liked songs...");

    Ok(())
}