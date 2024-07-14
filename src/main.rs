use std::path::Path;

use serde::{Deserialize, Serialize};
use ytranscript::YoutubeTranscript;

#[derive(Debug, Deserialize, Serialize)]
struct VideoData {
    pub items: Vec<VideoDescription>,
}

#[derive(Debug, Deserialize, Serialize)]
struct VideoDescription {
    pub snippet: Snippet,
}

#[derive(Debug, Deserialize, Serialize)]
struct Snippet {
    title: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VideoResult {
    pub short_name: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
struct VideoCSV {
    video_id: String,
}

async fn read_csv_video_url() -> Vec<VideoCSV> {
    let path = Path::new("./input.csv");
    let mut reader = csv::Reader::from_path(path).expect("Please add input.csv into main folder");
    let mut result = Vec::new();
    for parse_result in reader.deserialize() {
        let record: VideoCSV = parse_result.unwrap();
        result.push(record);
        println!("{:?}", result);
    }

    result
}

async fn save_videos_info(video: VideoResult) {
    let full_path = format!("./output/{}.txt", video.short_name);
    let path = Path::new(&full_path);
    tokio::fs::write(path, video.content).await.unwrap();
}

fn create_short_name_from_title(video_data: VideoData) -> String {
    let video_title = &video_data.items.first().unwrap().snippet.title;
    let title: String = video_title
        .trim()
        .to_lowercase()
        .replace(' ', "_")
        .chars()
        .filter(|ch| ch.is_alphabetic() || *ch == '_')
        .collect::<String>();
        
    title
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let video_list = read_csv_video_url().await;
    for video in video_list {
        let video_url = format!("https://www.youtube.com/watch?v={}", &video.video_id);
        let api_key = std::env::var("API_KEY").expect("Pls specify API_KEY for YOUTUBE connection");
        let video_url_title = format!(
            "https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=snippet,contentDetails",
            &video.video_id, api_key
        );
        println!("{video_url_title}");
        // Fetch the transcript
        let video_description: VideoData = reqwest::get(&video_url_title)
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        let short_title = create_short_name_from_title(video_description);
        println!("{short_title:?}");

        match YoutubeTranscript::fetch_transcript(&video_url, None).await {
            Ok(transcript) => {
                let mut result = String::new();
                for entry in transcript {
                    result += &format!(" {}", entry.text)[..];
                }
                let video_info = VideoResult{
                    short_name: short_title,
                    content: result
                };
                save_videos_info(video_info).await;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
