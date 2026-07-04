# BergCalc

The goal of this project is to create a fully open-source and replicable graphing calculator as a learning/teaching tool and  as a functional alternative to the likes of the TI-84. The project is currently in the software only phase, but eventually the goal is to have a no-std version of the rust library running with graphics on a Raspberry Pi Pico. 

There are 3 seperate projects contained in this repository:
- Decimal-math, a fully custom 32 digit precision decimal library. It is very buggy right now, not sure if this will be used in the final product.
- Python calculator (I don't have a better name for this), the python prototype of the calculator source code
- Rust calculator, the actual codebase for the eventual finished product, still very unfinished but making good progress.

# TODO

- [x] Basic rust parser
- [x] Basic rust evaluator
- [ ] Symbolic differentiation 
- [ ] Integration (Gauss-Kronrod)
- [ ] Expression solver using Newtons method
- [ ] Port parser/evaluator to use decimal-math library
- [ ] Start work on calculator GUI with embedded_graphics
- [ ] Basic evaluator GUI
- [ ] Graphing GUI
- [ ] Numeric solver GUI
- [ ] USB keyboard input
- [ ] Button matrix input
