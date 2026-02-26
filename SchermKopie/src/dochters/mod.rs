///Diverse opties zoals de kleur van pijlen, of het maken van een schermgrote afbeelding met een speciale knop.
/// Het optievenster kan worden verplaatst of gesloten als dat voor het maken van een selectie wenselijk is.
pub mod optie_venster;
/// Globale variabelen kunnen in rust alleen op bijzondere wijze worden gedefnieerd, dat gebeurt hier.
/// Globale variabelen wordt in rust afgeraden, omdat ze onbedoelde gevolgen hebben in heel andere delen van de programmatuur.
/// Niettemin hebben we een flink aantal globale variabelen bnodig in dit project
/// Het gebruik van globale variabelen (er zijn diverse typen in rust) gebeurt met bijzondere 'instrumenten'
pub mod globaal;
/// Een grote verzameling functies die elders worden gebruikt
pub mod gereedschap;
/// Het pijlenvenster maakt het mogelijk pijlen toe te voegen met daarbij bijschriften. Plaats van pijl-punt en pijl-achterzijde kunnen worden aangepast met de muis.
pub mod pijlen_app;
/// De pijlen-editor wordt zichtbaar als de schacht van een pijl wordt aangeklikt; hiermee kunnen bijschriften en kleur van de pijlen worden gewijzigd. 
pub mod pijlen_editor; 
/// het berichtvenster kan een opgegeven bericht in een extra venster weergeven
pub mod bericht_venster;
/// als de cursor op het niet-actieve_scherm komt wordt met een bericht-venster gewaarschuwd dat het andere scherm beschikbaar is voor schermkopie 
pub mod andere_scherm;
pub mod uitsnede_app;
pub mod bediening;
pub mod text_input;
