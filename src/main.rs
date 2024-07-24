use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use tokio;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct KanaMapping {
    kana_type: String,
    category: String,
    kana: String,
    romaji: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // 加載 .env 文件中的環境變數
    dotenv().ok();

    // 從環境變數中讀取 MongoDB 連接資訊
    let username = env::var("MONGO_USERNAME").expect("MONGO_USERNAME must be set");
    let password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD must be set");
    let cluster = env::var("MONGO_CLUSTER").expect("MONGO_CLUSTER must be set");

    let client_uri = format!(
        "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority&appName=Cluster0",
        username, password, cluster
    );
    let mut client_options = ClientOptions::parse(&client_uri).await?;

    // 設定 Stable API 版本
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // 取得 cluster 的 handle
    let client = Client::with_options(client_options)?;

    // Ping server 以確認連接成功
    client.database("admin").run_command(doc! {"ping": 1}, None).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    // 選擇資料庫和集合
    let db = client.database("jp_syllabaries");
    let collection: Collection<KanaMapping> = db.collection("kana_mappings");

    // 準備批次插入的資料
    let kana_mappings = vec![
        // 平假名清音 (Hiragana Seion)
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "あ".to_string(), romaji: "a".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "い".to_string(), romaji: "i".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "う".to_string(), romaji: "u".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "え".to_string(), romaji: "e".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "お".to_string(), romaji: "o".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "か".to_string(), romaji: "ka".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "き".to_string(), romaji: "ki".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "く".to_string(), romaji: "ku".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "け".to_string(), romaji: "ke".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "こ".to_string(), romaji: "ko".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "さ".to_string(), romaji: "sa".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "し".to_string(), romaji: "shi".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "す".to_string(), romaji: "su".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "せ".to_string(), romaji: "se".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "そ".to_string(), romaji: "so".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "た".to_string(), romaji: "ta".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ち".to_string(), romaji: "chi".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "つ".to_string(), romaji: "tsu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "て".to_string(), romaji: "te".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "と".to_string(), romaji: "to".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "な".to_string(), romaji: "na".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "に".to_string(), romaji: "ni".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ぬ".to_string(), romaji: "nu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ね".to_string(), romaji: "ne".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "の".to_string(), romaji: "no".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "は".to_string(), romaji: "ha".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ひ".to_string(), romaji: "hi".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ふ".to_string(), romaji: "fu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "へ".to_string(), romaji: "he".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ほ".to_string(), romaji: "ho".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ま".to_string(), romaji: "ma".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "み".to_string(), romaji: "mi".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "む".to_string(), romaji: "mu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "め".to_string(), romaji: "me".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "も".to_string(), romaji: "mo".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "や".to_string(), romaji: "ya".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ゆ".to_string(), romaji: "yu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "よ".to_string(), romaji: "yo".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ら".to_string(), romaji: "ra".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "り".to_string(), romaji: "ri".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "る".to_string(), romaji: "ru".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "れ".to_string(), romaji: "re".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ろ".to_string(), romaji: "ro".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "わ".to_string(), romaji: "wa".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "を".to_string(), romaji: "wo".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "seion".to_string(), kana: "ん".to_string(), romaji: "n".to_string() },

        // 片假名清音 (Katakana Seion)
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ア".to_string(), romaji: "a".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "イ".to_string(), romaji: "i".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ウ".to_string(), romaji: "u".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "エ".to_string(), romaji: "e".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "オ".to_string(), romaji: "o".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "カ".to_string(), romaji: "ka".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "キ".to_string(), romaji: "ki".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ク".to_string(), romaji: "ku".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ケ".to_string(), romaji: "ke".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "コ".to_string(), romaji: "ko".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "サ".to_string(), romaji: "sa".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "シ".to_string(), romaji: "shi".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ス".to_string(), romaji: "su".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "セ".to_string(), romaji: "se".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ソ".to_string(), romaji: "so".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "タ".to_string(), romaji: "ta".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "チ".to_string(), romaji: "chi".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ツ".to_string(), romaji: "tsu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "テ".to_string(), romaji: "te".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ト".to_string(), romaji: "to".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ナ".to_string(), romaji: "na".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ニ".to_string(), romaji: "ni".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ヌ".to_string(), romaji: "nu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ネ".to_string(), romaji: "ne".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ノ".to_string(), romaji: "no".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ハ".to_string(), romaji: "ha".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ヒ".to_string(), romaji: "hi".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "フ".to_string(), romaji: "fu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ヘ".to_string(), romaji: "he".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ホ".to_string(), romaji: "ho".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "マ".to_string(), romaji: "ma".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ミ".to_string(), romaji: "mi".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ム".to_string(), romaji: "mu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "メ".to_string(), romaji: "me".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "モ".to_string(), romaji: "mo".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ヤ".to_string(), romaji: "ya".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ユ".to_string(), romaji: "yu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ヨ".to_string(), romaji: "yo".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ラ".to_string(), romaji: "ra".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "リ".to_string(), romaji: "ri".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ル".to_string(), romaji: "ru".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "レ".to_string(), romaji: "re".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ロ".to_string(), romaji: "ro".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ワ".to_string(), romaji: "wa".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ヲ".to_string(), romaji: "wo".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "seion".to_string(), kana: "ン".to_string(), romaji: "n".to_string() },

        // 平假名拗音 (Hiragana Youon)
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "きゃ".to_string(), romaji: "kya".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "きゅ".to_string(), romaji: "kyu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "きょ".to_string(), romaji: "kyo".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "しゃ".to_string(), romaji: "sha".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "しゅ".to_string(), romaji: "shu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "しょ".to_string(), romaji: "sho".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "ちゃ".to_string(), romaji: "cha".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "ちゅ".to_string(), romaji: "chu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "ちょ".to_string(), romaji: "cho".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "にゃ".to_string(), romaji: "nya".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "にゅ".to_string(), romaji: "nyu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "にょ".to_string(), romaji: "nyo".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "ひゃ".to_string(), romaji: "hya".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "ひゅ".to_string(), romaji: "hyu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "ひょ".to_string(), romaji: "hyo".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "みゃ".to_string(), romaji: "mya".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "みゅ".to_string(), romaji: "myu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "みょ".to_string(), romaji: "myo".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "りゃ".to_string(), romaji: "rya".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "りゅ".to_string(), romaji: "ryu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "youon".to_string(), kana: "りょ".to_string(), romaji: "ryo".to_string() },

        // 片假名拗音 (Katakana Youon)
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "キャ".to_string(), romaji: "kya".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "キュ".to_string(), romaji: "kyu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "キョ".to_string(), romaji: "kyo".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "シャ".to_string(), romaji: "sha".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "シュ".to_string(), romaji: "shu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ショ".to_string(), romaji: "sho".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "チャ".to_string(), romaji: "cha".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "チュ".to_string(), romaji: "chu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "チョ".to_string(), romaji: "cho".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ニャ".to_string(), romaji: "nya".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ニュ".to_string(), romaji: "nyu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ニョ".to_string(), romaji: "nyo".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ヒャ".to_string(), romaji: "hya".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ヒュ".to_string(), romaji: "hyu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ヒョ".to_string(), romaji: "hyo".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ミャ".to_string(), romaji: "mya".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ミュ".to_string(), romaji: "myu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "ミョ".to_string(), romaji: "myo".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "リャ".to_string(), romaji: "rya".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "リュ".to_string(), romaji: "ryu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "youon".to_string(), kana: "リョ".to_string(), romaji: "ryo".to_string() },

        // 平假名濁音與半濁音 (Hiragana Dakuon and Handakuon)
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "が".to_string(), romaji: "ga".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぎ".to_string(), romaji: "gi".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぐ".to_string(), romaji: "gu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "げ".to_string(), romaji: "ge".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ご".to_string(), romaji: "go".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ざ".to_string(), romaji: "za".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "じ".to_string(), romaji: "ji".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ず".to_string(), romaji: "zu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぜ".to_string(), romaji: "ze".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぞ".to_string(), romaji: "zo".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "だ".to_string(), romaji: "da".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぢ".to_string(), romaji: "ji".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "づ".to_string(), romaji: "zu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "で".to_string(), romaji: "de".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ど".to_string(), romaji: "do".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ば".to_string(), romaji: "ba".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "び".to_string(), romaji: "bi".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぶ".to_string(), romaji: "bu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "べ".to_string(), romaji: "be".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぼ".to_string(), romaji: "bo".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぱ".to_string(), romaji: "pa".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぴ".to_string(), romaji: "pi".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぷ".to_string(), romaji: "pu".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぺ".to_string(), romaji: "pe".to_string() },
        KanaMapping { kana_type: "hiragana".to_string(), category: "dakuon_handaon".to_string(), kana: "ぽ".to_string(), romaji: "po".to_string() },

        // 片假名濁音與半濁音 (Katakana Dakuon and Handakuon)
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ガ".to_string(), romaji: "ga".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ギ".to_string(), romaji: "gi".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "グ".to_string(), romaji: "gu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ゲ".to_string(), romaji: "ge".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ゴ".to_string(), romaji: "go".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ザ".to_string(), romaji: "za".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ジ".to_string(), romaji: "ji".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ズ".to_string(), romaji: "zu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ゼ".to_string(), romaji: "ze".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ゾ".to_string(), romaji: "zo".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ダ".to_string(), romaji: "da".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ヂ".to_string(), romaji: "ji".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ヅ".to_string(), romaji: "zu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "デ".to_string(), romaji: "de".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ド".to_string(), romaji: "do".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "バ".to_string(), romaji: "ba".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ビ".to_string(), romaji: "bi".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ブ".to_string(), romaji: "bu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ベ".to_string(), romaji: "be".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ボ".to_string(), romaji: "bo".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "パ".to_string(), romaji: "pa".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ピ".to_string(), romaji: "pi".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "プ".to_string(), romaji: "pu".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ペ".to_string(), romaji: "pe".to_string() },
        KanaMapping { kana_type: "katakana".to_string(), category: "dakuon_handaon".to_string(), kana: "ポ".to_string(), romaji: "po".to_string() },
    ];

    // 執行批次插入
    collection.insert_many(kana_mappings, None).await?;

    println!("Inserted data successfully!");
    Ok(())
}
