#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in releasepk[nr].kleur

mod dochters;

use crate::dochters::{gereedschap::{cmd_line_argumenten,grootstescherm, monitor_wissel_functie},
globaal::{DOELSCHERM, KLEUR}, optie_venster::lees_ini_file, uitsnede_app::uitsnede_app};

/// De hoofdfunctie
pub fn main() {
    /*
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

        fn kwadraat(q: f32) -> f32 { q*q}  
        println!( "Q2 =  {:.2}", kwadraat(-5.0));

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

        let beest: &str;
        beest = "varken";
        let groter = "groter".to_string() + beest; 
        println!("Beest is een {}  {}", beest, groter);

        std::process::exit(0);
    // */    
// enkele functies die handig zijn bij het programmeren
    //env::set_var("RUST_BACKTRACE", "1");                // bij een fout worden meer stappen getoond, onoverzichtelijk maar inzicht gevend
    //tools::bert::yyy(KLEUR.get());                      // een testfunctie, om standaardfuncties in tools.rs te kunnen maken
// enkele voorbereidingen
    lees_ini_file();                                    // optie-file wordt ingelezen; de optie-file wordt herschreven als in het optie-venster wijzigingen worden ingevoerd.
    cmd_line_argumenten();                              // als een beeld-file op de command-line is opgegeven zal de uitsnede-functie niet worden gebruikt
    DOELSCHERM.set(grootstescherm());                   // het grootste scherm wordt gekozen om daaruit een screenshot te maken
// de hoofd-functies
    //uitsnede_of_pijlen();
    loop {   // het kan zijn dat we nog niet klaar zijn!
        uitsnede_app();
        monitor_wissel_functie();
    }    
}

/*
Gewenste uitbreidingen:
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
