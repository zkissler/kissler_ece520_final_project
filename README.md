# kissler_ece520_final_project

Port a non-trivial amount of Elma to Rust (e.g. Manager, Process, Event and some examples)

Project Goals:
The goal for this project is to complete option 4 of porting a non-trivial amount of Elma to Rust. I chose this project because I am interested in learning another language as well as understanding how languages have changed/improved since Rust is relatively new compared to C/C++, etc. Success will include not only a summary of preceived improvements and impedements that Rust provides as well as a functional version of Elma. I will note major difficulties as I work through the project. Resources I will require are the Elma code (for porting) as well as a Rust development environment.

Project Milestones:

3/6 - Become familiar with Rust and set up the build environment with Rust
3/9 - Create the code framework (ie: create class outlines) and make sure it compiles
3/13 - Port over code for the Manager, Process, Event and examples used in class
3/16 - Write/port tests for the examples and execute them (may need to research Rust test environments)
3/19 - Port over and test additional classes (Channel, State, etc) as time permits
3/21 - Submit completed project

Building a Docker for use with Rust:
docker build -t elma .
docker run -it --rm --name elma elma

Build a "crate" using "cargo build"
Run with "cargo run"

Current issues:
I have not figured out a clean way to implement class heirarchy in Rust. In C++ we have access to functions from the parent class but there is not a nice way to do that, at least that I have found yet. I tried implementing a trait for each process type (like BasicProcess) with the start, stop, init, and update functions however when passing a Process object into the Manager schedule function there was no way to access the higher level BasicProcess implementation. At this point I was able to make it work for the update function by having just one update function in the process implementation which looked at an enum and depending on what process type it was assigned it performed a different update function. This is ugly but I have spend 5+ hours trying to find a way around it and have not been successful yet.

Week 9 Update:

As of 3/13 I am right on track according to my project milestones. I have ported the Process, Manager, and Event classes to Rust. I have successfully tested basic process behavior and am in the process of working through how to test the Event class. I am very concerned that I will not be able to implement events as they are currently done in Elma due to the fact that I am unable to replicate the class inheritance used in the C++ version. I also attemped to use Doxygen with Rust but I have not been able to get Doxygen to recognize tags in my source file. 

I was able to get the scheduler working with two basic processes that simply print their respective process names every time the udpate function is called. The main.rs file has all of the tests in it, making use of manager.rs, process.rs, basic_process.rs and basic_process2.rs.
