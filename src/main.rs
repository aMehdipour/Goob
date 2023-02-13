#![allow(non_snake_case)]
#![windows_subsystem = "windows"]
mod matlab_script;

use eframe::{egui::Vec2, run_native, NativeOptions};
use futures::executor::block_on;
use matlab_script::MatlabScript;
use reqwest::Client;
use serde::Deserialize;
use std::ascii::AsciiExt;
use tokio::runtime::Runtime;

#[derive(Deserialize, Debug)]
struct PlaylistItem {
    id: String,
    snippet: Snippet,
}

#[derive(Deserialize, Debug)]
struct Snippet {
    title: String,
    description: String,
    resourceId: ResourceId,
}

#[derive(Deserialize, Debug)]
struct ResourceId {
    videoId: String,
}

#[derive(Deserialize, Debug)]
struct PlaylistResponse {
    items: Vec<PlaylistItem>,
    nextPageToken: Option<String>,
}

async fn get_playlist_items(playlist_id: &str, api_key: &str) -> Vec<(String, String)> {
    let client = Client::new();
    let mut playlist_items = vec![];
    let mut next_page_token = None;

    loop {
        let response = client
            .get(&format!(
                "https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&maxResults=999&playlistId={}&key={}{}",
                playlist_id,
                api_key,
                next_page_token
                    .as_ref()
                    .map(|token| format!("&pageToken={}", token))
                    .unwrap_or_else(|| "".to_owned())
            ))
            .send()
            .await
            .unwrap();

        let playlist_items_response: PlaylistResponse = response.json().await.unwrap();
        let items = playlist_items_response
            .items
            .into_iter()
            .filter(|item| {
                item.snippet.title != ""
                    && !item
                        .snippet
                        .title
                        .trim()
                        .eq_ignore_ascii_case("Deleted video")
            })
            .map(|item| {
                (
                    item.snippet.title,
                    format!(
                        "https://www.youtube.com/watch?v={}",
                        item.snippet.resourceId.videoId
                    ),
                )
            })
            .collect::<Vec<(String, String)>>();
        playlist_items.extend(items);

        match playlist_items_response.nextPageToken {
            Some(token) => next_page_token = Some(token),
            None => break,
        }
    }

    playlist_items
}

fn main() {
    let playlist_id = "PLbo9hNXX38Ytwm5oe6S9S8SBrDTNwvoHF";
    let api_key = "AIzaSyCQ-rbP3bqFkJMk8eO5xZpX3y33u3OoBxo";
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let mut titles: Vec<String> = vec![];
    let mut urls: Vec<String> = vec![];

    let playlist_items = rt.block_on(get_playlist_items(playlist_id, api_key));

    for item in playlist_items {
        titles.push(item.0.clone());
        urls.push(item.1.clone());
    }

    let app = MatlabScript::new(titles, urls);
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(780., 960.));
    run_native(Box::new(app), win_option);
}
