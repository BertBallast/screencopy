#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in releasepk[nr].kleur

mod dochters;

use crate::dochters::{gereedschap::{cmd_line_argumenten, monitor_wissel_functie, hoofdscherm},
globaal::DOELSCHERM, optie_venster::lees_ini_file, uitsnede_app::uitsnede_app};

/// De hoofdfunctie
pub fn main() {
    /*   plaats nog een tweede '/' voor de eerste '/' om deze rust-testjes uit te voeren 
        let s= "aaāp".to_string();   // 4 unicode charas
        let r= "aaāp";
        let b: Vec<char>= s.chars().collect();   // 5 bytes
        println!("STR {}, BYT  {}", s.len(), b.len());
        //for i in s[0..s.len()] {println!("BB {}", s[i]);}
        for c in b {println!("C= {}  {}", c, c as usize);}
        let f= 2.54;
        let q: f32 = f;
        let t= "varken";
        let s= t.to_string();
        let s= String::from(t);
        let mut y= 32;
        let m= & mut y;
        *m += 8;
        println!("Q= {}   M={}", q, m);
       //assert!(*m== 40);
        //assert!(y == 40);
        //let k= y;
        let p= *m;
        let h= 4.3;
        let k= h as i32;
        let j= k as f32;
        println!("*m={}  p={} k={}  h= {} j= {:.3}", *m, p, k, h, j);
        let mut a= vec![7, 14, 3];
        let mut b= a.clone();
        a[1]= 28;
        b.push(8);
        println!("A= {:?}  B= {:?}", a, b);
        
        let a= "aap".to_string();
        let b= a.clone();
        println!("a= {}   b= {}", a, b);

        let a= 2; let b= 1;
        let k= if a>b {4} else {6};
        println!("K= {} want a>b= {}", k, a>b);

        //fn kwadraat(q: f32) -> f32 { q*q}  
        //println!( "Q2 =  {:.2}", kwadraat(-5.0));

        let tekst= "5.0";
        let lengte= tekst.parse::<f32>();
        println!("Lengte= {:?}", lengte);
        if lengte.is_ok() {
            println!("lengt= {}", lengte.clone().unwrap());
        }
        match lengte {
            Ok(l)=> {
                println!("lengte= {l}");
            },
            Err(e) => {
                println!("Fout: {}", e);
            }
        }

        let beest: &str= "varken";
        let groter = "groter".to_string() + " " + beest; 
        println!("Beest is een {}  {}", beest, groter);

        let mut klein: String= beest.to_string() + "tje";
        klein= klein+ "s";
        println!("zie hier: {klein}");

        let sss= "12kg".parse::<f32>();
        println!("{:?}", "12kg".parse::<f32>());
        println!("{:?}", sss);

let pi: f32;
pi= 3.14;
let straal: f64 = 25.4;
let mut getal: Option<i32>;
getal = Some(57);
print!("PI: {} omtrek: {}  {:?}", pi, 2.0 * pi * straal as f32, getal);
getal = None;
let vee= klein.clone();
let pi: f32= 3.14; let straal= 10; let omtrek= 2.0 * pi * straal as f32;
//let mut kwadraat4= 4.0; kwadraat4= kwadraat4 * kwadraat4;
//println! ("     getal:  {:?}  {}  {}  4^2={:.3}", getal, beest, klein, kwadraat4);
//let mut hoeveel: Option<f32>= None;
//hoeveel= Some(kwadraat4);

let beest: &str= "varken"; let mut kleintjes= beest.to_string(); kleintjes= kleintjes+ "tjes";
let slachtvee= kleintjes.clone(); kleintjes= slachtvee + " worden groot";
println!("Vee: {}", kleintjes);

let mut a= 8; let mut b=2;
if a>b {let c= a; a= b; b= c;}
println!("begin met klein  {a}  {b}");

let a= 2; let b= 1; let k= if a<b {4} else {6}; 
println!("zes: {k}");

fn kwadraat (q: i32) -> i32 {q*q}; println! ("kwadraat van 7= {}", kwadraat(7));
      
let tekst= "12.3kg"; 
let gewicht = tekst.parse::<f32>();  //  tekst omzetten in getal
match gewicht {                                 // lengte kan getal zijn, maar ook een fout (error)
             Ok(gew)=> {println!("gewicht= {gew}");},      // 12.3 als ‘kg’ er niet zou staan, maar het wordt:
     Err(e) => { println!("Fout: {}", e);}                    // Err(ParseFloatError {  kind : Invalid } )  
}

        std::process::exit(0);
// */    // einde rust-testjes
    //env::set_var("RUST_BACKTRACE", "1");                // bij een fout worden meer stappen getoond, onoverzichtelijk maar inzicht gevend

    // enkele voorbereidingen
    lees_ini_file();                                    // optie-file wordt ingelezen; de optie-file wordt herschreven als in het optie-venster wijzigingen worden ingevoerd.
    cmd_line_argumenten();                              // als een beeld-file op de command-line is opgegeven zal de uitsnede-functie niet worden gebruikt
    DOELSCHERM.set(hoofdscherm());                   // het grootste scherm wordt gekozen om daaruit een screenshot te maken
// de hoofd-functie
    loop {   // het kan zijn dat we nog niet klaar zijn!
        uitsnede_app();
        // als de uitsnede bekend is wordt aamn het begin van uitnsede_app overgechakeld naar pijlen_app
        // als uitsnede_app gesloten moet worden geven we een 'eframe::egui::ViewportCommand::Close' in de functie 'naar_pijlen' in 'uitsnede_app.rs'
        monitor_wissel_functie();
    }    
}

/*
Gewenste toekomstige uitbreidingen:
1. mogelijkkheid om uit bestaand beeld een uitsnede te maken en daarna pijlen toe te voegen
2. een beeld opslaan in een kleinere resolutie naar keuze
3. tonen van de omvang van een uitsnede in pixels
4. een bijschrift maken zonder pijl, bijv als een pijl met verwaarloosbare lengte wordt gemaakt
5. handleiding langduriger zichtbaar maken op verzoek
*/

/*
Bronnen:
https://github.com/gfx-rs/wgpu
https://github.com/emilk/egui/blob/master/examples/multiple_viewports/src/main.rs
https://stackoverflow.com/questions/75655361/how-to-get-a-screenshot-of-a-specific-window-with-rust-on-windows
https://stackoverflow.com/questions/73096821/take-a-screenshot-of-more-than-one-screen-connected-to-the-same-device-in-rust-l
*/

/*
Extra viewport cannot be transparent:
https://github.com/emilk/egui/discussions/4735
https://github.com/emilk/egui/issues/3632
*/
