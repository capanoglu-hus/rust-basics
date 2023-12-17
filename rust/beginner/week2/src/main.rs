fn main() {
   

    let music1 = Music {
        name: String::from("Dönence"),
        artist: String::from("Barış Manço "),
        active: true,
        time: 3,
        tur:TurE::rock,
    };

    let music2 = Music {
        name: String::from("Ceviz agacı"),
        artist: String::from("Cem karaca  "),
        active: false,
        time: 3,
        tur:TurE::rock,
    };

    let music3 = Music {
        name: String::from("Believer"),
        artist: String::from("Imagine Dragons"),
        active: false,
        time: 3,
        tur:TurE::pop,
    };
 
    let music4 = Music {
        name: String::from("we will rock you "),
        artist: String::from("Queen "),
        active: true,
        time: 3,
        tur:TurE::rock,
    }; 
   println!(" music1 ---->");
   print_music_info(&music1);

   println!(" music2 ---->");
   print_music_info(&music2);

   println!(" music3 ---->");
   print_music_info(&music3);

   println!(" music4 ---->");
   print_music_info(&music4);

   let mut playlist1: Vec<Music> = Vec::new();

   playlist1.push(music1);
   playlist1.push(music2);
   playlist1.push(music3);
   println!("Playlist1:");
    for music in &playlist1 {
    
        print_music_info(music);
    }
    let mut playlist2: Vec<Music> = Vec::new();

    
   playlist2.push(music4);
    println!("Playlist 2 :");
     for music in &playlist2 {
     
        print_music_info(music);
     }

     calinan_music_info(&playlist1);

     calinan_music_info(&playlist2);
 } 

 fn calinan_music_info(playlist: &Vec<Music>) {
    for music in playlist {
        if music.active == true {
            println!("  şuan çalınan şarkı :: " );
            print_music_info(music);
        }
    }
}


 fn print_music_info(music: &Music) {
    println!(
        " Name: {} \n Artist: {} \n TİME: {} \n tür: {:?} \n active  {} \n" ,
        music.name, music.artist, music.time, music.tur , music.active,
    );
}
#[derive(Debug)]
 struct Music {
    name:String,
    artist:String,
    tur:TurE,
    active:bool,
    time:i32,
   }

   #[derive(Debug)]
   enum TurE {
    pop,
    rock  ,
    caz,
    klasik ,
   }