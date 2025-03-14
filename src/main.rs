use gtfs_structures::Gtfs;
use std::io::Write;
use std::io::Read;
use std::fs::File;

#[tokio::main]
async fn main() {
   let gtfs_url = "https://cdn.rtcquebec.ca/Site_Internet/DonneesOuvertes/googletransit.zip";

   //save to temp.zip

    let temp_zip = "temp.zip";

    let mut response = reqwest::get(gtfs_url).await.unwrap();

    let mut file = std::fs::File::create(temp_zip).unwrap();

    while let Some(chunk) = response.chunk().await.unwrap() {
        file.write_all(&chunk).unwrap();
    }

    //read gtfs structures, if okay, save as rtcquebec_latest.zip

    let gtfs = Gtfs::new(temp_zip).unwrap();

    let rtcquebec_latest = "rtcquebec_latest.zip";

    std::fs::copy(temp_zip, rtcquebec_latest).unwrap();

    //delete temp.zip

    std::fs::remove_file(temp_zip).unwrap();
}
