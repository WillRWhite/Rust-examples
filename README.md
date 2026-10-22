# Rust-examples

Useful Resources
----------------
Installer: https://rust-lang.org/tools/install  
Learn: https://www.rust-lang.org/learn/get-started  
Docs: https://doc.rust-lang.org/stable/  

Rust Dependencies  
-----------------  
On Windows Rust needs MSVC build tools (see: https://claude.ai/share/05e9ce86-c912-4f07-8c78-2dddc134a9b7)  
They can be installed here:- https://visualstudio.microsoft.com/downloads/    
*Scroll to “Tools for Visual Studio”. In the installer, tick "Build Tools for Visual Studio 202x" and select "Desktop development with C++". This includes the MSVC compiler and linker and the Windows SDK.*    


Rust commands  
-------------  
**rustup** :- installer and toolchain manager  
`rustup update` :-  *run this frequently*     
`rustup doc` :- *doc appears in a web browser*    

**cargo** :- package manager and build tool  
`cargo new project_name_no_extension` *git normally initilised unless already in a git repo but can be omitted by creating a config.toml file somewhere in the project hirarchy*    
`cargo check`  
`cargo build` :- *output emds up in the ./target directory*  
`cargo run` :- *builds, if necessary and rnns from the target directory*  

**rustc** :- complles a "standalone" rust file to the current directory  
**rustfmt** :- Format source code  

