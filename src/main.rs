// Didn't use git throughout the whole making of this...sorry for that :(

// The code most likely isn't clean by Rustacean standards, but it at least works...

use macroquad::prelude::*;

const INITIAL_AMOUNT: usize = 64;
const BLOB_RATE: usize = 1;

struct Blob {
    size: f32,
    speed: f32,
    pos_x: f32,
    pos_y: f32,
    color: Color
}

fn push_new_blob(mut blobs: Vec<Blob>) -> Vec<Blob> {
    blobs.push(Blob{
            size: rand::gen_range(10.0, 50.0),
            speed: rand::gen_range(0.0, 10.0),
            pos_x: rand::gen_range(0.0, screen_width()),
            pos_y: -100.0,
            color: Color {
                a: rand::gen_range(0.0, 1.0), 
                b: rand::gen_range(0.0, 1.0), 
                g: rand::gen_range(0.0, 1.0), 
                r: rand::gen_range(0.0, 1.0)}
        }
    );

    return blobs;
}


#[macroquad::main("blobmark_rs")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    
    let mut blobs: Vec<Blob> = vec![];

    for _i in 0..INITIAL_AMOUNT {
        blobs = push_new_blob(blobs); 
    }

    loop {
        clear_background(BLACK);

        for blob in &mut blobs {
            draw_circle(blob.pos_x, blob.pos_y, blob.size, blob.color);
            blob.pos_y += blob.speed;
        }
        
        for _i in 0..BLOB_RATE {
            blobs = push_new_blob(blobs);
        }
        
        let dumbass_string = format!("Blob count: {}", blobs.len().to_string());
        draw_text(&dumbass_string, 0.0, 40.0, 20.0, WHITE);
        draw_text("Every blob is updated every frame, after which one new blob gets added (assuming the BLOB_RATE const is set to 1).", 0.0, screen_height() - 30.0, 15.0, WHITE);
        draw_text("The blob count includes blobs that aren't visible on screen, as blobs off-screen will still exist in memory.", 0.0, screen_height() - 20.0, 15.0, WHITE);
        draw_text("Made by mrxbox1 on 16 May 2026", 0.0, screen_height() - 10.0, 15.0, WHITE);
        draw_fps();

        next_frame().await;
    }
}
