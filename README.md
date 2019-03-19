# kissler_ece520_final_project

Port a non-trivial amount of Elma to Rust (e.g. Manager, Process, Event and some examples)

Project Goals:
The goal for this project was to complete option 4 of porting a non-trivial amount of Elma to Rust. I chose this project because I was interested in learning another language as well as understanding how languages have changed/improved since Rust is relatively new compared to C/C++, etc. 

Installation, Build, & Run - NEED TO FIX

Building a Docker for use with Rust:
docker build -t elma .
docker run -it --rm --name elma elma

Build a "crate" using "cargo build"
Run with "cargo run"

Build the documentation using:
cargo doc
And open the <current_directory>/target/doc/main/index.html

Build and automatically open the documentation:
cargo doc --open

Conclusion:
Three weeks was not nearly enough time to properly dig into this project. This project has required learning a whole new language on top of porting the Elma code to Rust. There are numerous nuances about the Rust language that vastly differ from how things are handled in C. I found these to be incredibly difficult to wrap my head around in such a short period of time on top of completing the class homework assignments. A major downfall of using a new language is that online resources were generally lacking. I found answers to basic questions and common problems but I had trouble tracking down examples on how to do more complex tasks whereas you can easily find solutions for C/C++ issues online. I feel like Rust is a very powerful language but with minimal resources available it was much more challenging than I thought it would be to port Elma to Rust.

Successes:
Once I began to understand the syntax used in Rust I began creating the process class. I had a fairly easy time implementing the basics of the process class as well as the manager class. Rust includes a majority of the same storage types that C++ has and they were fairly easy to interact with. I had trouble at first understanding the difference between mutable and immutable variables but ultimately I understand how the use of these types of variables can help prevent memory leaks as Rust is a memory safe language.

A major positive I experienced with Rust was the warnings and errors reported at compile time were very helpful. They often included hints as to how the problem could be solved. Obviously learning a new language came with a significant amount of compiler errors.

I was also able to somewhat implement class inheritance through the use of traits. This allowed me to have a different implementation of the init, start, stop, and update functions for each process. However this method required wrapping everything in a trait object to be stored in the manager module which did not allow for access to the process structures themselves. Also this resulted in the repition of code and each process structure had to implement all the member variables instead of inheriting them. This code is not currently used in the main.rs file but if you comment out the first block of code and uncomment the second block it will use this method to run with the two basic processes I implemented in test.rs.

Failures:
Major failures of this project involved the more complex differences between Rust and C++. I was unable to implement "virtual" functions in the process class that could be overwritten by functions in the higher-level class. C++ allows users to "extend" classes and override functions but Rust does not have that same sort of Parent/Child class relationship. I had to use an enum in the process implementation in each init, start, stop, and update function. I spent most of the first week researching a way around this with no luck. I originally had minor success with this but I could not access or update the variables in the process structures themselves.

I also struggled to understand how to pass references and pointers much like what is common in C and C++. This proved to be an issues when trying to implement the "manager_ptr" that the process class has back to the manager class. I kept getting a "moved out of borrowed content" error and had to eventually store a clone of the manager instance. I tried to use raw pointers in my structures but I ran into errors like "does not have size at compile time" or "requires lifetime paramater". I researched using lifetime paramters but could not get to a point where I felt like I had a solid understanding in the limited time that I had left.

Due to the failures I experienced I did my best to implement extra Elma classes to make up for this. Most of these classes ran on the foundation of class inheritance so once again I had trouble actually testing these implementations in the same way that the standard Elma does. I am confident that with more time and a better understanding of how to implement inheritance I would have been able to successfully port all of Elma to Rust.

Future Recommendations:
If this option continues to be used in future classes I would provide some sort of primer on implementing inheritance in Rust. I spent 20+ hours researching inheritance alone (about 50 hours overall)and despite repeated attempts I recieved no help from the professor on this topic. Ultimatley it does not make sense to have students to port Elma to Rust without more guidance.
