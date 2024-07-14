use serde::{Deserialize, Serialize};
use ytranscript::YoutubeTranscript;

#[derive(Debug, Deserialize, Serialize)]
struct VideoData {
    pub items: Vec<VideoDescription>
}

#[derive(Debug, Deserialize, Serialize)]
struct VideoDescription {
    pub snippet: Snippet
}

#[derive(Debug, Deserialize, Serialize)]
struct Snippet {
    title: String,
    description: String,
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let video_id = "qy1eijEkwFU";
    let video = format!("https://www.youtube.com/watch?v={}", video_id);
    let api_key = std::env::var("API_KEY").expect("Pls specify API_KEY for YOUTUBE connection");
    let video_url_title = format!("https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=snippet,contentDetails", video_id, api_key);
    // Fetch the transcript
    let video_description: VideoData = reqwest::get(video_url_title).await.unwrap().json().await.unwrap();
    println!("{video_description:?}");

    match YoutubeTranscript::fetch_transcript(&video, None).await {
        Ok(transcript) => {
            let mut result = String::new();
            for entry in transcript {
                result +=  &format!(" {}", entry.text)[..];
            }
            println!("{:?}", result);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
