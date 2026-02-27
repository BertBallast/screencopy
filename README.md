# screencopy
A screenshot program, in RUST, with features to add arrows and annotations. The code is provided, as well as an executable for windows. The first purpose of the project is to check the suitability of the rust language for end-user applications. 

Rust is a system programming language focusing on safety, speed and concurrency. The same source code can be compiled under several operating systems. Executables run without a separate 'runtime environment'. These properties draw my intention.

But, is the language suitable for end-user applications too? To answer that question, this project was started, using some available crates, e.g. epframe.rs for a grahic user interface.
The answer: Yes, much is possible, but there are some unexpected drawbacks, see 'remarks' below.

NB: Comments in the code are in dutch, and also the names of most variables - However, in the near future it will be translated to englisch
NB: 'Rust by example', the most accessible way to learn the language, has many small examples. This project shows a complete application.

Remarks
* The crate mouse-rs is used. In linux Debian and Ubuntu it needs installation of the library libxdo-dev with the command ‘sudo apt-get install libxdo-dev’
* In linux-wayland the function ‘.with_position’ of the egui-ViewportBuilder is documented as ‘unsupported’. This means that all viewports are positioned in the center of the screen.
* In linux-X11 there is some inconsistency in the use of ‘scale’ and ‘pixels_per_point’. This needs some extra attention.
* In linux-wayland the ‘Virtual Keyboard’ should be set to ‘None’, in ‘System settings>Keyboard>Virtual Keyboard’

