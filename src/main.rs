use chrono::prelude::*;
use tokio::process::Command;
use egg_mode::{KeyPair, Token, raw::{request_post, response_json, ParamList}, Response};
use serde_json::Value;
use serde::Deserialize;
use anyhow::Result;

const SOURCE_IMAGE: &str = "kumassy-icon-small.jpg";
const TMP_IMAGE: &str= "rotated.jpg";

#[derive(Deserialize, Debug)]
struct Config {
  consumer_key: String,
  consumer_secret: String,
  access_token_key: String,
  access_token_secret: String
}

fn get_angle<T: TimeZone>(today: Date<T>) -> u32 {
    let progress: f64 = (today.ordinal0() as f64) / 365.0;
    let angle: u32 = (progress * 360.0).floor() as u32;
    angle
}

async fn upload_image(config: Config, img: String) -> Result<()> {
    let con_token = KeyPair::new(config.consumer_key, config.consumer_secret);
    let access_token = KeyPair::new(config.access_token_key, config.access_token_secret);
    let token = Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let params = ParamList::new().add_param("image", img);
    let request = request_post("https://api.twitter.com/1.1/account/update_profile_image.json", &token, Some(&params));
    let _json: Response<Value> = response_json(request).await?;
    Ok(())
}

fn get_base64(path: &str) -> Result<String> {
    let img = image::open(path)?;

    let mut buf = vec![];
    img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(80))?;
    Ok(base64::encode(&buf))
}

async fn convert_image(angle: u32) -> Result<String> {
    let output = Command::new("convert")
        .arg(SOURCE_IMAGE)
        .arg("-distort")
        .arg("SRT")
        .arg(format!("+{}", angle))
        .arg(TMP_IMAGE)
        .output()
        .await?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = envy::from_env::<Config>().expect("missing env vars.");
    let today_utc: Date<Utc> = Utc::today();
    let today_jst: Date<FixedOffset> = today_utc.with_timezone(&FixedOffset::east(9*3600));

    let angle = get_angle(today_jst);
    println!("today: {}", today_jst);
    println!("today's angle: {}", angle);

    convert_image(angle).await.expect("failed to convert image. ImageMagic is not installed?");
    let img = get_base64(TMP_IMAGE).expect("failed to open converted image.");
    upload_image(config, img).await.expect("failed to upload image.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_get_angle() {

        let base = Utc.ymd(2000, 1, 1);

        for offset in 0..100000 {
            let day = base + Duration::days(offset);
            let angle = get_angle(day);
            assert!(angle <= 360);
        }
    }
}